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

    let mut priority: Vec<u32> = Vec::new();

    // Read all the lines of the file
    if let Ok(lines) = read_lines(input_path) {
        // Go throug each of the lines
        for line in lines {

            // Error handling
            match line {
                Ok(value) => {
                    // Extract the compartments
                    let first_compartment: String = value[0..(value.chars().count()/2)].to_string();
                    let second_compartment: String = value[(value.chars().count()/2)..value.chars().count()].to_string();

                    // Find the shared items
                    let mut shared_items: Vec<char> = Vec::new();

                    // Go through all the items in the first compartment
                    for item in first_compartment.chars() {
                        // If the second compartment has the same item, and we haven't already counted the item add it to the list of shared items
                        if second_compartment.contains(item) && !shared_items.contains(&item) {
                            shared_items.push(item);
                        }
                    }

                    for item in &shared_items {
                        if item.is_lowercase() {
                            priority.push(item.to_digit(36).unwrap() - 9); // 'a' radix = 10, so we remove 9 for it to be 1. We then get 'a'=1, 'b'=2, etc.
                        } else {
                            priority.push(item.to_digit(36).unwrap() + 17); // 'A' radix = 10, so we add 17 for it to be 27. We then get 'A'=27, 'B'=28, etc.
                        }
                    }

                    //println!("{}\n{}\n{}\n{:?}\n{:?}\n{}", value, first_compartment, second_compartment, &shared_items, &priority, &priority.iter().sum::<u32>());
                    //println!("");
                }
                Err(_) => {}
            }
        }
    }

    println!("The total priority for the elves is: {}", priority.iter().sum::<u32>());
}

fn convert_to_priotiy(items: &Vec<char>) -> u32 {
    let mut priority: Vec<u32> = Vec::new();

    for item in items {
        if item.is_lowercase() {
            priority.push(item.to_digit(36).unwrap() - 9); // 'a' radix = 10, so we remove 9 for it to be 1. We then get 'a'=1, 'b'=2, etc.
        } else {
            priority.push(item.to_digit(36).unwrap() + 17); // 'A' radix = 10, so we add 17 for it to be 27. We then get 'A'=27, 'B'=28, etc.
        }
    }

    return priority.iter().sum();
}

fn task_two() {
    let input_path = Path::new("input");

    let mut elf_groups: Vec<(String, String, String)> = Vec::with_capacity(3);
    let mut elf_group: (String, String, String) = (String::new(), String::new(), String::new());

    // Read all the lines of the file
    if let Ok(lines) = read_lines(input_path) {
        // Go throug each of the lines
        for (i, line) in lines.enumerate() {

            // If the number is divisible with three, new group

            // Error handling
            match line {

                // Sort the elves into groups of three
                Ok(value) => {
                    match i%3 {
                        0 => {
                            if i != 0{ // Make sure it's not the first iteration
                                // If it is divisible by 3 we have a new group so we want to push the last group to the list before flushing.
                                elf_groups.push(elf_group);
                                elf_group = ("".to_string(), "".to_string(), "".to_string());
                            }
                            elf_group.0 = value;
                        }
                        1 => { elf_group.1 = value; }
                        2 => { elf_group.2 = value; }
                        _ => {}
                    }
                }
                Err(_) => { println!("Error, this should never happen :("); }
            }
        }
    }
    // Make sure the last group is included as well.
    elf_groups.push(elf_group);
    println!("{:#?}, {}", elf_groups, elf_groups.len());
    
    let mut common_items: Vec<char> = Vec::new();
    // Find the common items in the groups
    for group in &elf_groups {
        // Loop through all the items in the first elves backpack
        for item in group.0.chars() {
            // See if the other elves have it in theirs
            if group.1.contains(item) && group.2.contains(item) {
                // Add it to the vector of common items
                common_items.push(item);
                break;
            }
        }
    }

    let priority = convert_to_priotiy(&common_items);
    println!("The group priority is {priority}");

}

fn main() {
    task_two();
}
