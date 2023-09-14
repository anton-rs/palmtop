// use std::process;

use clap::Parser;
// use dirs::home_dir;
use eyre::Result;

use palmtop_telemetry::{self, metrics};

// use serde::Serialize;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let logs_dir = cli.logs_dir.clone();
    let logs_rotation = cli.logs_rotation.clone();
    let verbose = cli.verbose;

    let _guards = palmtop_telemetry::init(verbose, logs_dir, logs_rotation);
    metrics::init()?;

    tracing::info!(target: "palmtop", "Starting Magi...");

    // let runner = Runner::from_config(config)
    //     .with_sync_mode(sync_mode)
    //     .with_checkpoint_hash(checkpoint_hash);
    //
    // if let Err(err) = runner.run().await {
    //     tracing::error!(target: "magi", "{}", err);
    //     process::exit(1);
    // }

    Ok(())
}

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long, default_value = "optimism")]
    network: String,
    #[clap(long)]
    l1_rpc_url: Option<String>,
    #[clap(long)]
    l2_rpc_url: Option<String>,
    #[clap(long)]
    l2_engine_url: Option<String>,
    #[clap(long)]
    jwt_secret: Option<String>,
    #[clap(short = 'v', long)]
    verbose: bool,
    #[clap(short = 'p', long)]
    rpc_port: Option<u16>,
    #[clap(long)]
    logs_dir: Option<String>,
    #[clap(long)]
    logs_rotation: Option<String>,
    #[clap(long)]
    checkpoint_hash: Option<String>,
    #[clap(long)]
    checkpoint_sync_url: Option<String>,
}

// impl Cli {
//     pub fn to_config(self) -> Config {
//         let chain = match self.network.as_str() {
//             "optimism" => ChainConfig::optimism(),
//             "optimism-goerli" => ChainConfig::optimism_goerli(),
//             "optimism-sepolia" => ChainConfig::optimism_sepolia(),
//             "base" => ChainConfig::base(),
//             "base-goerli" => ChainConfig::base_goerli(),
//             file if file.ends_with(".json") => ChainConfig::from_json(file),
//             _ => panic!(
//                 "Invalid network name. \\
//                 Please use one of the following: 'optimism', 'optimism-goerli', 'base-goerli'. \\
//                 You can also use a JSON file path for custom configuration."
//             ),
//         };
//
//         let config_path = home_dir().unwrap().join(".magi/magi.toml");
//         let cli_config = CliConfig::from(self);
//         Config::new(&config_path, cli_config, chain)
//     }
// }
//
// impl From<Cli> for CliConfig {
//     fn from(value: Cli) -> Self {
//         Self {
//             l1_rpc_url: value.l1_rpc_url,
//             l2_rpc_url: value.l2_rpc_url,
//             l2_engine_url: value.l2_engine_url,
//             jwt_secret: value.jwt_secret,
//             checkpoint_sync_url: value.checkpoint_sync_url,
//             rpc_port: value.rpc_port,
//         }
//     }
// }
