use clap::{App, AppSettings, Arg, ArgMatches};

pub fn get_matches() -> ArgMatches {
    let matches = App::new("revelation")
        .version("0.1")
        .about("Elevation GRPC api using geotiff data")
        .subcommand(App::new("run").arg(Arg::new("config_path").default_value("config.toml")))
        .setting(AppSettings::ArgRequiredElseHelp);

    matches.get_matches()
}
