use serde_json::Value;
use structs::client::Client;

mod args;
mod funcs;
mod json;
mod structs;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init_custom_env("rranch_log");
    let mut client = Client::new("localhost", 27015)?;
    client.auth("hirn", "CONTROLLER", "default", 0)?;
    client.checkout("crosstools")?;
    Ok(())
}
