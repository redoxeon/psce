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
        matrix.set_slice(i, &Matrix::new(1, 1, secret_algorithm(
            driver_names.lines().nth(i.0).unwrap(),
            dest_addrs.lines().nth(i.1).unwrap(),
        )));
    });

    dbg!(&matrix);

    let (_total, distribution) = kuhn_munkres(&matrix);

    let mut main_list: Vec<OutputLine> = Vec::new();
    for (i, driver) in driver_names.lines().enumerate() {
        let current_line: OutputLine = OutputLine {
            driver: driver.to_owned(),
            shipment_destination: dest_addrs.lines().nth(distribution[i]).unwrap().to_owned(),
            ss: matrix.get((i, distribution[i])).unwrap().clone(),
        };

        main_list.push(current_line);
    }

    Ok(main_list)
}

fn secret_algorithm(driver: &str, addr: &str) -> i32 {
    let mut ss: f64;
    let street_name = trim_to_street_name(addr);

    if street_name.len() % 2 == 0 {
        ss = find_vowels(driver) as f64 * 1.5;
    } else {
        ss = find_consonants(driver) as f64;
    }

    // additional common factors related to length can be added here
    if street_name.len() == driver.to_owned().len() {
        ss *= 1.5;
    }

    ss as i32
}

fn trim_to_street_name(addr: &str) -> String {
    // take off everything after the street name, at the first comma
    let mut street_name = addr.to_owned();
    street_name.truncate(street_name.find(',').unwrap());

    // take off the street number, before the first space
    let num_offset = street_name.find(' ').unwrap();
    street_name.drain(..(num_offset + 1));

    street_name
}

fn find_consonants(my_string: &str) -> i32 {
    let mut cons_count = 0;
    let re = Regex::new(r"(?i)[bcdfghjklmnpqrstvwxyz]").ok().unwrap();
    for _mat in re.find_iter(my_string) {
        cons_count += 1;
    }
    cons_count
}

fn find_vowels(my_string: &str) -> i32 {
    let mut vowel_count = 0;
    let re = Regex::new(r"(?i)[aeiou]").ok().unwrap();
    for _mat in re.find_iter(my_string) {
        vowel_count += 1;
    }
    vowel_count
}

#[test]
fn test_trim() {
    let addr = "123 Elm Street, Scary Town, IN, 55555";
    assert_eq!(String::from("Elm Street"), trim_to_street_name(addr));
}

#[test]
fn test_consonants() {
    let addr = "Elm Street";
    assert_eq!(6, find_consonants(addr));
}

#[test]
fn test_vowels() {
    let addr = "Elm Street";
    assert_eq!(3, find_vowels(addr));
}

#[test]
fn test_secret_alg() {
    let addr = "123 Elm Street, Scary Town, IN, 55555";
    let name = "Jane Doe";
    assert_eq!(6, secret_algorithm(name, addr));
}
