use crate::config;
use gdal::raster;
use gdal::spatial_ref;
use gdal::Dataset;

struct CoordTransform {
    pub orig: gdal::spatial_ref::CoordTransform,
}
unsafe impl Send for CoordTransform {}
unsafe impl Sync for CoordTransform {}

pub struct Entry<'a> {
    pub source: &'a config::Source,
    pub dataset: gdal::Dataset,
    coord_trans: CoordTransform,
    // band: raster::RasterBand<'a>,
    geo_transform_inv: (f64, f64, f64, f64, f64, f64),
    no_data_value: f64,
    // band_x_size: f64,
    pub xmax: f64,
    pub xmin: f64,
    pub ymax: f64,
    pub ymin: f64,
}
unsafe impl Send for Entry<'_> {}
unsafe impl Sync for Entry<'_> {}

pub fn from_dataset(source: &config::Source, dataset: Dataset) -> Entry {
    // Get geo_transform
    let gt = dataset.geo_transform().unwrap();

    // Get coord trans instance
    let dataset_spatial_ref = dataset.spatial_ref().unwrap();
    let target_spatial_reference = spatial_ref::SpatialRef::from_epsg(4326).unwrap();
    let base_coord_trans =
        spatial_ref::CoordTransform::new(&target_spatial_reference, &dataset_spatial_ref).unwrap();
    let coord_trans = CoordTransform {
        orig: base_coord_trans,
    };

    // Compute corners
    let [xmin, xpixel, _, ymax, _, ypixel] = gt;
    let (width, height) = dataset.raster_size();
    let xmax = xmin + width as f64 * xpixel;
    let ymin = ymax + height as f64 * ypixel;

    // Convert dataset coords to EPSG:4326
    coord_trans
        .orig
        .transform_coords(&mut [ymax], &mut [xmax], &mut [0.])
        .unwrap();
    coord_trans
        .orig
        .transform_coords(&mut [ymin], &mut [xmin], &mut [0.])
        .unwrap();

    // Compute geo transforms
    let dev = gt[1] * gt[5] - gt[2] * gt[4];
    let geo_transform_inv = (
        gt[0],
        gt[5] / dev,
        -gt[2] / dev,
        gt[3],
        -gt[4] / dev,
        gt[1] / dev,
    );

    // Load band
    let band = dataset.rasterband(1).unwrap();

    // no data value
    let no_data_value = band.no_data_value().unwrap();

    // band_x_size
    // let band_x_size = band.x_size() as f64;

    Entry {
        source,
        dataset,
        // band,
        coord_trans,
        geo_transform_inv,
        no_data_value,
        // band_x_size,
        xmax,
        xmin,
        ymax,
        ymin,
    }
}

impl<'a> Entry<'a> {
    /// get altitude from entry
    pub fn get_altitude(&self, lat: f64, lng: f64) -> Option<f64> {
        // Convert coords
        let lat = &mut [lat];
        let lng = &mut [lng];
        let alt = &mut [0.0];
        self.coord_trans
            .orig
            .transform_coords(lng, lat, alt)
            .unwrap();

        // convert it to pixel/line on band
        let u = lat[0] - self.geo_transform_inv.0;
        let v = lng[0] - self.geo_transform_inv.3;
        let xpix = (self.geo_transform_inv.1 * u + self.geo_transform_inv.2 * v).floor() as isize;
        let ylin = (self.geo_transform_inv.4 * u + self.geo_transform_inv.5 * v).floor() as isize;
        // let image_index = xpix.round() + self.band_x_size * ylin.round();

        // Load band
        let band = self.dataset.rasterband(1).unwrap();

        // Read only pixel in band
        let image_data: raster::Buffer<f64> =
            band.read_as((xpix, ylin), (1, 1), (1, 1), None).unwrap();
        println!("{:?}", image_data.data);
        let result = image_data.data[0];

        if result == self.no_data_value {
            return None;
        }

        Some(result)
    }

    // Check if raster contain given point
    pub fn contain_point(&self, lat: f64, lng: f64) -> bool {
        // Convert coords
        let lat = &mut [lat];
        let lng = &mut [lng];
        let alt = &mut [0.0];
        self.coord_trans
            .orig
            .transform_coords(lng, lat, alt)
            .unwrap();

        // Check
        lat[0] >= self.xmin && lat[0] <= self.xmax && lng[0] >= self.ymin && lng[0] <= self.ymax
    }
}
