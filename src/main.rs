use log::LevelFilter;
use koradi_svc::server;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    match server::run() {
        Ok(_) => println!("Server ran successfully"),
        Err(e) => eprintln!("Server error: {:?}", e),
    }
}

