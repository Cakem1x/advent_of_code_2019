use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// day 1
fn get_fuel_requirements_by_mass(mass: u32) -> u32 {
    if mass < 9 {
        return 0;
    } else {
        let third_of_mass_rounded_down = mass / 3;
        let fuel_requirements = third_of_mass_rounded_down - 2;
        return fuel_requirements + get_fuel_requirements_by_mass(fuel_requirements);
    }
}

fn main() -> io::Result<()> {
    let input_file = File::open("input.txt").expect("couldn't read file input.txt");
    let reader = BufReader::new(&input_file);
    let mut fuel_requirements_sum = 0;
    for line in reader.lines() {
        let current_module_mass: u32 = line.unwrap().parse().unwrap();
        let current_fuel_requirements = get_fuel_requirements_by_mass(current_module_mass);
        println!(
            "Mass: {}, fuel-requirements: {}",
            current_module_mass, current_fuel_requirements
        );
        fuel_requirements_sum += current_fuel_requirements;
    }
    println!("Sum of fuel requirements: {}", fuel_requirements_sum);
    Ok(())
}
