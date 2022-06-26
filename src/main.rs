use env_logger::Builder;
use std::path::PathBuf;
use std::time::Duration;
use tonic::transport::Server;

mod cmd;
pub mod config;
mod server;
pub mod tree;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new()
        .filter_level(log::LevelFilter::Info)
        .parse_env("RELEVATION_LOG")
        .init();

    // Parse cmd
    let cmd_matches = cmd::get_matches();
    if let Some(ref cmd_matches) = cmd_matches.subcommand_matches("run") {
        // Load config
        let config_path = cmd_matches.value_of("config_path");
        if config_path.is_none() {
            panic!("Config file required");
        }

        let config_path_buf = PathBuf::from(config_path.unwrap());
        let mut cfg = config::new();
        match cfg.load_from(&config_path_buf) {
            Ok(v) => v,
            Err(_e) => panic!("Failed to load configuration file: {}", _e),
        };

        // Create a new tree
        let mut tr = tree::Tree::new();

        // Use scope to let tr rwlock be unlocked before server start, else tr is never unlocked
        // for reading
        {
            // Build tree
            for source in cfg.sources.iter() {
                // let ss: config::Source = source.clone();

                log::info!("Load source with id {} in memory ...", source.id);

                match tr.load_source(source) {
                    Ok(v) => v,
                    Err(_e) => panic!("Failed to load source : {}", _e),
                };
            }
        }

        // Start GRPC server
        let addr = cfg.host().clone().parse()?;
        let service = server::RelevationService::new(tr, 10);

        log::info!("Staring GRPC server on {}", cfg.host().clone());

        Server::builder()
            .concurrency_limit_per_connection(32)
            .timeout(Duration::from_secs(30))
            .add_service(server::relevation::relevation_server::RelevationServer::new(service))
            .serve(addr)
            .await?;

        return Ok(());
    }

    Ok(())
}
