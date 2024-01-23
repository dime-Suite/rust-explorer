mod config;
pub mod constants;
mod database;
pub mod logger;
pub mod server;

use clap::Parser;
use log::{debug, info};

// using CLAP rust for cli management
#[derive(Parser, Debug)]
#[command(name = "RUST CAT EXPLORER")]
#[command(author = "Jayendra M. <jayendramadaram@gmail.com>")]
#[command(version = "0.0.0")]
#[command(
    about = "friedly cat explorer",
    long_about = "Catalof orderbook explorer built as a side project Collab: @Nesopie"
)]
struct Cli {
    #[clap(short = 'c', long = "config", help = "Configuration file path" , default_value = constants::DEFAULT_CONFIG_FILE)]
    config_file: String,
    // #[clap(short = 'd', long = "db", help = "DB path" , default_value = constants::DEFAULT_DB_PATH)]
    // db_path: String,
}

#[tokio::main]
async fn main() {
    debug!("Starting up...");
    let cli_args = Cli::parse();
    let config = config::Config::create_or_load_config(&cli_args.config_file)
        .expect("Failed to Load Config");
    logger::init(&config.logger.enable_level);
    let db = database::Database::open(config.db.max_connections, &config.db.db_uri)
        .await
        .expect("Failed to Open DB");
    info!("Database connection established.");
    server::boostrap_server(config, db).await;
    // TODO: open db connection
    // TODO: start server
}
