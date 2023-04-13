use structs::client::Client;

mod args;
mod funcs;
mod json;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    pretty_env_logger::init_custom_env("rranch_log");
    let mut client = Client::new("localhost", 27015)?;
    client.auth("hirn", "CONTROLLER", "default", 0)?;
    Ok(())
}
