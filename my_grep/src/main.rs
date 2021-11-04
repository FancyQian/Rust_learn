use std::env;
use std::process;
use my_grep::GrepConfig;

#[macro_use] extern crate log;

fn main() {
    // set default log level
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    env_logger::init();

    let config = GrepConfig::new(env::args()).unwrap_or_else(|err| {
        error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    info!("Ready to running...");

    let reslut = config.run().unwrap_or_else(|err| {
        error!("Running Error: {}", err);
        process::exit(1);
    });

    println!("{:?}", reslut);
}