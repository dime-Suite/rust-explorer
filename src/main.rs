mod config;
pub mod constants;
pub mod logger;
pub mod model;
pub mod server;

use anyhow::Result;
use axum::Router;
use clap::Parser;
use log::{debug, info};

use model::ModelManager;
use server::routes::orderbook;
use tokio::net::TcpListener;

// using CLAP rust for cli management
#[derive(Parser, Debug)]
#[command(name = "RUST CAT EXPLORER")]
#[command(author = "Jayendra M. <jayendramadaram@gmail.com>")]
#[command(version = "0.0.0")]
#[command(
    about = "friedly cat explorer",
    long_about = "Catalog orderbook explorer built as a side project Collab: @Nesopie"
)]
struct Cli {
    #[clap(short = 'c', long = "config", help = "Configuration file path" , default_value = constants::DEFAULT_CONFIG_FILE)]
    config_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    debug!("Starting up...");

    let cli_args = Cli::parse();

    let config = config::Config::create_or_load_config(&cli_args.config_file).unwrap();

    logger::init(&config.logger.enable_level);
    let mm = ModelManager::new(config.db.max_connections, &config.db.db_uri).await?;

    info!("Database connection established.");

    let routes = Router::new().merge(orderbook::routes(mm));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}
