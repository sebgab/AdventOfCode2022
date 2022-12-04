use std::path::Path;
use std::fs::{File};
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn task_one() {
    let input_path = Path::new("input");
    let mut elf_pairs_and_zones: Vec<(Vec<u64>, Vec<u64>)> = Vec::new();
    let mut elf_pair: (Vec<u64>, Vec<u64>) = (Vec::new(), Vec::new());
    let mut fully_contained_counter: u64 = 0;

    // Read all the lines of the file
    if let Ok(lines) = read_lines(input_path) {

        // Go throug each of the lines
        for line in lines {

            // Error handling
            match line {
                Ok(value) => {
                    for (i, s) in value.split(',').enumerate() { // Split the string at the comma
                        let range = s.split('-').collect::<Vec<&str>>(); // Split the resulting string at the dash
                        
                        let start = range[0].parse::<u64>().unwrap();   // Get the start value out of the split number
                        let end = range[1].parse::<u64>().unwrap();     // Get the end value out of the split number

                        // Add all the values in the range to the vector in elf_pair
                        match i%2 {
                            0 => { for i in start..end+1 { elf_pair.0.push(i); }}
                            1 => { for i in start..end+1 { elf_pair.1.push(i); }}
                            _ => {}
                        }
                    }
                    // Push and reset the elf_pair vector
                    elf_pairs_and_zones.push(elf_pair);
                    elf_pair = (Vec::new(), Vec::new());
                }
                Err(_) => {}
            }
        }
    }

    for zones in elf_pairs_and_zones {
        // Extract the data
        let pattern_0 = &zones.0;
        let pattern_1 = &zones.1;

        let mut duplicate_zones = 0;

        // Check the first zone
        for zone in pattern_0 {
            if pattern_1.contains(zone) {
                duplicate_zones += 1;
            }
        }
        // See if all the zones are containes
        if duplicate_zones == pattern_0.len() {
            fully_contained_counter += 1;
        } else { // If it is not check the second zone
            duplicate_zones = 0;
            for zone in pattern_1 {
                if pattern_0.contains(zone) {
                    duplicate_zones += 1;
                }
            }
            // See if all the zones are containes
            if duplicate_zones == pattern_1.len() {
               fully_contained_counter += 1;
            }
        }
    }

    println!("There are {fully_contained_counter} fully contained duplicate zones in the pairs.");
}

fn task_two() {
    let input_path = Path::new("input");
    let mut elf_pairs_and_zones: Vec<(Vec<u64>, Vec<u64>)> = Vec::new();

    // Read all the lines of the file
    if let Ok(lines) = read_lines(input_path) {

        // Go throug each of the lines
        for line in lines {

            // Error handling
            match line {
                Ok(value) => {
                    let mut elf_pair: (Vec<u64>, Vec<u64>) = (Vec::with_capacity(1), Vec::with_capacity(1));
                    for (i, s) in value.split(',').enumerate() { // Split the string at the comma
                        let range = s.split('-').collect::<Vec<&str>>(); // Split the resulting string at the dash
                        
                        let start = range[0].parse::<u64>().unwrap();   // Get the start value out of the split number
                        let end = range[1].parse::<u64>().unwrap();     // Get the end value out of the split number

                        // Add all the values in the range to the vector in elf_pair
                        match i%2 {
                            0 => { for i in start..end+1 { elf_pair.0.push(i); }}
                            1 => { for i in start..end+1 { elf_pair.1.push(i); }}
                            _ => {}
                        }
                    }
                    // Push and reset the elf_pair vector
                    elf_pairs_and_zones.push(elf_pair);
                }
                Err(_) => {}
            }
        }
    }

    let mut pairs_with_duplicates = 0;
    for zones in elf_pairs_and_zones {
        // Extract the data
        let pattern_0 = &zones.0;
        let pattern_1 = &zones.1;

        // Check if there is a duplicate in the pair
        for zone in pattern_0 {
            if pattern_1.contains(zone) { 
                // If there is a duplicate add to the counter and stop the check.
                pairs_with_duplicates += 1;
                break;
            }
        }
    }

    println!("There are {pairs_with_duplicates} pairs with duplicate zones");
}

fn main() {
    task_one();
    task_two();
}
