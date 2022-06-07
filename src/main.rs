use pathfinding::prelude::{kuhn_munkres, Matrix};
use regex::Regex;
use std::fs;
use std::io;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct OutputLine {
    driver: String,
    shipment_destination: String,
    ss: i32,
}

fn main() {
    println!("First, please provide the path to a list of destination addresses");
    let mut file = String::new();
    io::stdin()
        .read_line(&mut file)
        .expect("Failed to read path");
    let addresses = fs::read_to_string(&file.trim()).expect("Error opening addresses file.");

    let mut file = String::new();
    println!("Next, please provide the path to the list of driver names");
    io::stdin()
        .read_line(&mut file)
        .expect("Failed to read path");
    let drivers = fs::read_to_string(&file.trim()).expect("Error opening drivers file.");

    if let Ok(output) = hungarian_algorithm_helper(addresses, drivers) {
        let table = Table::new(output).to_string();
        print!("{table}");
    }
}

fn hungarian_algorithm_helper(
    dest_addrs: String,
    driver_names: String,
) -> Result<Vec<OutputLine>, std::io::Error> {
    let mut matrix = Matrix::new(driver_names.lines().count(), dest_addrs.lines().count(), -1);
    matrix.indices().for_each(|i| {
        let mut _cell = matrix.get_mut(i).unwrap();
        _cell = &mut secret_algorithm(
            driver_names.lines().nth(i.0).unwrap(),
            dest_addrs.lines().nth(i.1).unwrap(),
        )
        .unwrap();
    });

    let (_total, distribution) = kuhn_munkres(&matrix);

    let mut main_list: Vec<OutputLine> = Vec::new();
    for (i, driver) in driver_names.lines().enumerate() {
        let current_line: OutputLine = OutputLine {
            driver: driver.to_owned(),
            shipment_destination: dest_addrs.lines().nth(distribution[i]).unwrap().to_owned(),
            ss: matrix.idx((i, distribution[i])) as i32,
        };

        main_list.push(current_line);
    }

    Ok(main_list)
}

fn secret_algorithm(driver: &str, addr: &str) -> Option<i32> {
    let mut ss = 0.0;

    // take off everything after the street name, at the first comma
    let mut street_name = addr.to_owned();
    street_name.truncate(street_name.find(',')?);

    // take off the street number, before the first space
    let num_offset = street_name.find(' ')?;
    street_name.drain(..num_offset);

    if street_name.len() % 2 == 0 {
        let re = Regex::new(r"[aeiou]").ok()?;
        for _mat in re.find_iter(driver) {
            ss += 1.5;
        }
    } else {
        let re = Regex::new(r"[bcdfghjklmnpqrstvwxyz]").ok()?;
        for _mat in re.find_iter(driver) {
            ss += 1.0;
        }
    }

    // additional common factors related to length can be added here
    if street_name.len() == driver.to_owned().len() {
        ss *= 1.5;
    }

    if ss <= 0.0 {
        () // if no suitability score could be fount, return None
    }

    Some(ss as i32)
}
