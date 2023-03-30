use args::argparser::Arg;
use log::{error, info};

use crate::{args::argparser::ArgParser, connection::client::Client};
mod args;
mod connection;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init_custom_env("rranch_log");
    let mut ap = ArgParser::new(Vec::new(), None, Vec::new());
    ap.define_arg(Arg::new("t", "typ", "typo", None));
    ap.define_arg(Arg::new("m", "monn", "monnolo", Some("gaming".to_owned())));
    ap.parse_args();
    ap.help();
    println!("{:#?}", ap.get_parsed());
    let mut client = match Client::new(
        "localhost",
        27015,
        Some("hirn".to_string()),
        "schitcliönt".to_owned(),
        "CONTROLLER".to_owned(),
    ) {
        Ok(client) => client,
        Err(err) => {
            error!("Failed to connect to server: {err}");
            return Ok(());
        }
    };

    match client.auth() {
        Ok(()) => info!("Successfully authenticated"),
        Err(err) => error!("Failed to authenticate: {err}"),
    }

    match client.set_type() {
        Ok(()) => info!("Successfully set machine type"),
        Err(err) => error!("Failed to set machine type: {err}"),
    }

    match client.set_name() {
        Ok(()) => info!("Successfully set machine name"),
        Err(err) => error!("Failed to set machine name: {err}"),
    }

    match client.close_connection() {
        Ok(()) => info!("Successfully shut down connection"),
        Err(err) => error!("Failed to shut down connection: {err}"),
    }

    Ok(())
}
