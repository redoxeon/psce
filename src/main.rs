// The output should be the total suitability score and a matching between
// shipment destinations and drivers. You do not need to worry about malformed
// input, but you should certainly handle both upper and lower case names.

use std::io;

// Assumptions:
// - Each name and address is on it's own line
// - address layout:
//     111 street name, city, ST, 11111
// - name layout:
//     firstname lastname
// - output layout:
//     SS        Shipment Destination                Driver
//     00.00     111 street name, city, ST, 11111    firstname lastname
//     00.00     111 street name, city, ST, 11111    driverfirstname lastnamename
//     ...       ...                                 ...

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

    // If the length of the shipment's destination street name is even, the base
    // suitability score (SS) is the number of vowels in the driver's name
    // multiplied by 1.5.

    // If the length of the shipment's destination street name is odd, the base SS
    // is the number of consonants in the driver's name multiplied by 1.

    // If the length of the shipment's destination street name shares any common
    // factors (besides 1) with the length of the driver's name, the SS is increased
    // by 50% above the base SS.

}
