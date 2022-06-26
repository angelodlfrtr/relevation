use crate::config;
use gdal::Dataset;
use std::error::Error;
use std::path::PathBuf;

mod entry;
pub mod errors;
mod lib;

#[derive(Debug, Clone)]
pub struct ElevationResult {
    pub elevation: f64,
    pub dataset_id: String,
}

pub struct Tree<'a> {
    entries: Vec<entry::Entry<'a>>,
}

impl<'a> Default for Tree<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Tree<'a> {
    /// new Tree with given cache capacity
    pub fn new() -> Tree<'a> {
        Tree {
            entries: Vec::new(),
        }
    }

    // load_source load tiff files from source
    pub fn load_source(&mut self, source: &'a config::Source) -> Result<(), Box<dyn Error>> {
        let root_path = PathBuf::from(source.path.clone());

        // Check root validity (should exist and be a dir)
        lib::check_root(&root_path)?;

        // Find all geotiff files in dirs (recursive)
        let tif_paths = lib::list_geotif_files(&root_path)?;

        // Loop on files
        for pp in tif_paths.iter() {
            // Open dataset with gdal
            let dataset = Dataset::open(pp.as_path())?;

            // Build entry
            let etry = entry::from_dataset(source, dataset)?;

            self.entries.push(etry);
        }

        Ok(())
    }

    pub fn find_entries_containing_point(&self, lat: f64, lng: f64) -> Vec<&entry::Entry> {
        let mut result = Vec::new();

        for etry in self.entries.iter() {
            if etry.contain_point(lat, lng) {
                result.push(etry);
            }
        }

        result
    }

    pub fn find_entry_containing_point(
        &self,
        lat: f64,
        lng: f64,
        dataset_id: Option<String>,
    ) -> Option<&entry::Entry> {
        let etries = self.find_entries_containing_point(lat, lng);
        if etries.is_empty() {
            return None;
        }

        // Prefered dataset
        if let Some(dataset_id_string) = dataset_id {
            if !dataset_id_string.is_empty() {
                for etry in etries.iter() {
                    if etry.source.id == dataset_id_string {
                        return Some(etry);
                    }
                }
            }
        }

        // Else filter by resolution
        let mut result = etries[0];
        let mut last_resolution: usize = result.source.resolution;
        for etry in etries.iter() {
            if etry.source.resolution > last_resolution {
                result = etry;
                last_resolution = etry.source.resolution;
            }
        }

        Some(result)
    }

    /// get altitude from lat and lng
    pub fn get_altitude(
        &self,
        lat: f64,
        lng: f64,
        dataset_id: Option<String>,
    ) -> Option<ElevationResult> {
        let coords = &[lat, lng];
        let etry = self.find_entry_containing_point(coords[0], coords[1], dataset_id);

        match etry {
            Some(etry_val) => {
                let elevation_result = etry_val.get_altitude(coords[0], coords[1]);

                if let Some(elevation_result_value) = elevation_result {
                    // Build result
                    let result = ElevationResult {
                        elevation: elevation_result_value,
                        dataset_id: etry_val.source.id.clone(),
                    };

                    return Some(result);
                }

                None
            }
            None => None,
        }
    }
}
