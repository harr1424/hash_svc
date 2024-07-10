use koradi_svc::server;

fn main() {
    match server::run() {
        Ok(_) => println!("Server ran successfully"),
        Err(e) => eprintln!("Server error: {:?}", e),
    }
}

