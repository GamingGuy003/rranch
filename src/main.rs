use args::argparser::ArgParser;

mod args;
mod funcs;
mod json;
mod structs;
mod util;

fn main() -> std::io::Result<()> {
    let argparser = ArgParser::new(
        Vec::new(),
        Some("The branch client rewritten in Rust with Protocol version 2 (json)"),
        Vec::new(),
    );

    pretty_env_logger::init_custom_env("rranch_log");
    Ok(())
}
