use structs::client::Client;

mod args;
mod funcs;
mod json;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    std::env::set_var("rranch_log", "trace");
    pretty_env_logger::init_custom_env("rranch_log");
    let mut client = Client::new("", 27015)?;
    client.auth("rranch-testclient", "CONTROLLER", "", 0)?;
    //client.get_job_log("e4da4bb0-0922-4647-a20e-9b7e039c6a15")?;
    //client.checkout("thunar")?;
    //client.submit("./thunar/package.bpb")?;
    //client.build("thunar-volman", true)?;
    //client.get_sys_log()?;
    //client.get_dependers("thunar")?;
    //client.rebuild_dependers("thunar")?;
    //client.get_jobs_status(false)?;
    //client.get_latest_log()?;
    //client.get_sys_log()?;
    //client.watch_jobs("1")?;
    //client.get_clients()?;
    // client.get_managed_pkgs()?;
    // client.get_managed_pkgbs()?;

    Ok(())
}
