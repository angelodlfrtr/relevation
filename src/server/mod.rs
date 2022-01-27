use std::time::Instant;
use tonic::{Code, Request, Response, Status};

mod cache;

pub mod relevation {
    tonic::include_proto!("relevation");
}

use relevation::relevation_server::Relevation;
use relevation::{
    Empty, GetElevationInput, GetElevationOutput, GetElevationsInput, GetElevationsOutput, Point,
};

// #[derive(Debug, Default)]
pub struct RelevationService {
    tree: crate::tree::Tree,
    cache: cache::Cache,
}

impl RelevationService {
    /// new Relevation Service
    pub fn new(tree: crate::tree::Tree, cache_size: usize) -> RelevationService {
        RelevationService {
            tree,
            cache: cache::Cache::new(cache_size),
        }
    }
}

#[tonic::async_trait]
impl Relevation for RelevationService {
    async fn ping(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        let reply = Empty {};
        Ok(Response::new(reply))
    }

    async fn get_elevation(
        &self,
        request: Request<GetElevationInput>,
    ) -> Result<Response<GetElevationOutput>, Status> {
        let start = Instant::now();

        let input = request.into_inner();
        let input_point = input.point.unwrap();

        // Find elevation from tree
        let coords = &[input_point.lat, input_point.lng];

        // Papare point
        let mut point = None;

        // Check cache
        let cache_res = self
            .cache
            .get(coords[0], coords[1], Some(input_point.dataset_id.clone()));
        if cache_res.is_some() {
            let cc = cache_res.unwrap();

            point = Some(Point {
                elv: Some(cc.elevation),
                lat: input_point.lat,
                lng: input_point.lng,
                dataset_id: cc.dataset_id,
            })
        } else {
            // Not in cache

            // Get elevation from tree
            let result = self
                .tree
                .get_altitude(coords[0], coords[1], Some(input_point.dataset_id));
            if result.is_some() {
                let cc = result.unwrap();

                // Save in cache
                self.cache.add(coords[0], coords[1], cc.clone());

                point = Some(Point {
                    elv: Some(cc.elevation),
                    lat: input_point.lat,
                    lng: input_point.lng,
                    dataset_id: cc.dataset_id,
                })
            }
        }

        // Reply
        let reply = GetElevationOutput { point };

        log::debug!("GetElevation request took {}ns", start.elapsed().as_nanos());

        Ok(Response::new(reply))
    }

    /// Get elevation from request
    async fn get_elevations(
        &self,
        request: Request<GetElevationsInput>,
    ) -> Result<Response<GetElevationsOutput>, Status> {
        let start = Instant::now();

        // Prepare result
        let mut result_points = Vec::new();

        // Loop on input points
        let input = request.into_inner();

        // Check input points len
        if input.points.len() > 10000 {
            return Err(Status::new(
                Code::OutOfRange,
                "more than 10k points in request",
            ));
        }

        for pt in input.points.iter() {
            // Prepare point
            let mut point = Point {
                elv: None,
                lat: pt.lat,
                lng: pt.lng,
                dataset_id: "none".to_string(),
            };

            // Find correct datased in tree
            let coords = &[pt.lat, pt.lng];

            let result = self
                .tree
                .get_altitude(coords[0], coords[1], Some(pt.dataset_id.clone()));
            if result.is_some() {
                let cc = result.unwrap();

                point.elv = Some(cc.elevation);
                point.dataset_id = cc.dataset_id.clone();
            }

            result_points.push(point);
        }

        // Reply
        let reply = GetElevationsOutput {
            points: result_points,
        };

        log::debug!(
            "GetElevations request took {}ns",
            start.elapsed().as_nanos()
        );

        Ok(Response::new(reply))
    }
}
