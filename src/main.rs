use std::io;

struct Driver {
    name: String,
    current_destination: String,
}

struct Destination {
    addr: String,
    current_suitability_score: i32,
}

fn main() {
    // TODO get 2 files (one at a time)

    let dest_addrs_file = String::new();
    let driver_names_file = String::new();

}

fn secret_algorithm(dest_addrs: String, driver_names: String) -> Result<i32, std::io::Error> {

}
