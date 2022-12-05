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

    let mut cargo_stack: Vec<Vec<char>> = Vec::new(); // Vector to hold the cargo stacks
    let mut cargo_movement_orders: Vec<(u32, u32, u32)> = Vec::new(); // (amount, from, to)
    let mut order_flag = false; // Flag for when the order part has come
    
    // Read all the lines of the file
    if let Ok(lines) = read_lines(input_path) {

        // Go throug each of the lines
        for line in lines {

            // Error handling
            match line {
                Ok(value) => {
                    if !order_flag { // Check if we are loading orders or crate orders
                        if value == "" { order_flag = true; } else { // Check if we are transitioning from crates to orders

                            // Make sure that there is an equal amount of vectors in the cargo stack vector as there are cargo stacks in the task
                            let cargo_len = (&value.chars().count()+1)/4; // Each stack takes up 3 slots + the space between the stacks. Thus we need to add 1 for the missing last space and divide by 4 to get the amount of stacks.
                            while cargo_stack.len() < cargo_len { // Make sure the amount is the same
                                cargo_stack.push(Vec::new());
                            }

                            // Go through the chars in the line
                            for (i, char) in value.chars().enumerate() {
                                match i%4 {
                                    1 => { // This is where the cargo label is
                                        if char.is_numeric() { break; } // If it is a number we have reached the end of the cargo and reached the stack labels.
                                        if char == ' ' { continue; }    // If the label is empty we ignore it, we don't want a vector filled with ' '.

                                        let stack_num = i/4;   // Find the pallet number by dividing i by 4. Because it rounds down this works.
                                        cargo_stack[stack_num].push(char); // Push the cargo to the appropriate stack
                                    }
                                    _ => {}
                                }
                            }
                        }
                    } else {
                        let mut order = (0, 0, 0); // Create an empty order

                        let mut i: u32 = 0;                                     // i to keep track of which order we are working on.
                        for s in value.split(' ') {                       // Split it at the spaces
                            if s.chars().nth(0).unwrap().is_numeric() {         // Is the first character in the slice a number?
                                let value = s.parse::<u32>().unwrap();     // Parse the str into a number
                                match i {                                       // Put the number into the appropriate order place
                                    0 => { order.0 = value; i += 1; }
                                    1 => { order.1 = value; i += 1; }
                                    2 => { order.2 = value; i += 1; }
                                    _ => {}
                                }
                            }
                        }

                        cargo_movement_orders.push(order);                      // Add it to the vector of orders.
                    }
                }
                Err(_) => {}
            }
        }
    }

    // The stacks will be added top down, but we want them bottom up.
    for i in 0..cargo_stack.len() {
        cargo_stack[i].reverse();
    }

    // Apply the movement orders
    for order in cargo_movement_orders {
        for _ in 0..order.0 {
            // Extract the from and to orders
            let from = order.1 as usize - 1;
            let to = order.2 as usize - 1;

            let cargo = cargo_stack[from].pop().unwrap();   // Get the cargo out of the from stack
            cargo_stack[to].push(cargo);                          // Add it to the to stack
        }
    }

    // Find the top ones so we can report it back to the elves
    let mut top_crates: String = String::new();
    for stack in cargo_stack {
        // Get the top item from the stacks and add them to the top_crates string.
        top_crates.push(*stack.get(stack.len()-1).unwrap());
    }

    // Print the output to the user.
    println!("With CrateMover 9000 the resulting order is: {}", top_crates);

}

fn task_two() {
    let input_path = Path::new("input");

    let mut cargo_stack: Vec<Vec<char>> = Vec::new(); // Vector to hold the cargo stacks
    let mut cargo_movement_orders: Vec<(u32, u32, u32)> = Vec::new(); // (amount, from, to)
    let mut order_flag = false; // Flag for when the order part has come
    
    // Read all the lines of the file
    if let Ok(lines) = read_lines(input_path) {

        // Go throug each of the lines
        for line in lines {

            // Error handling
            match line {
                Ok(value) => {
                    if !order_flag { // Check if we are loading orders or crate orders
                        if value == "" { order_flag = true; } else { // Check if we are transitioning from crates to orders

                            // Make sure that there is an equal amount of vectors in the cargo stack vector as there are cargo stacks in the task
                            let cargo_len = (&value.chars().count()+1)/4; // Each stack takes up 3 slots + the space between the stacks. Thus we need to add 1 for the missing last space and divide by 4 to get the amount of stacks.
                            while cargo_stack.len() < cargo_len { // Make sure the amount is the same
                                cargo_stack.push(Vec::new());
                            }

                            // Go through the chars in the line
                            for (i, char) in value.chars().enumerate() {
                                match i%4 {
                                    1 => { // This is where the cargo label is
                                        if char.is_numeric() { break; } // If it is a number we have reached the end of the cargo and reached the stack labels.
                                        if char == ' ' { continue; }    // If the label is empty we ignore it, we don't want a vector filled with ' '.

                                        let stack_num = i/4;   // Find the pallet number by dividing i by 4. Because it rounds down this works.
                                        cargo_stack[stack_num].push(char); // Push the cargo to the appropriate stack
                                    }
                                    _ => {}
                                }
                            }
                        }
                    } else {
                        let mut order = (0, 0, 0); // Create an empty order

                        let mut i: u32 = 0;                                     // i to keep track of which order we are working on.
                        for s in value.split(' ') {                       // Split it at the spaces
                            if s.chars().nth(0).unwrap().is_numeric() {         // Is the first character in the slice a number?
                                let value = s.parse::<u32>().unwrap();     // Parse the str into a number
                                match i {                                       // Put the number into the appropriate order place
                                    0 => { order.0 = value; i += 1; }
                                    1 => { order.1 = value; i += 1; }
                                    2 => { order.2 = value; i += 1; }
                                    _ => {}
                                }
                            }
                        }

                        cargo_movement_orders.push(order);                      // Add it to the vector of orders.
                    }
                }
                Err(_) => {}
            }
        }
    }

    // The stacks will be added top down, but we want them bottom up.
    for i in 0..cargo_stack.len() {
        cargo_stack[i].reverse();
    }

    // Apply the movement orders
    for order in cargo_movement_orders {
        // Extract the from and to orders
        let amount = order.0 as usize;
        let from = order.1 as usize - 1;
        let to = order.2 as usize - 1;
        
        // Variables to make the code more readable
        let stack_length = cargo_stack[from].len();
        let start = stack_length-amount;

        let mut moving_cargo: Vec<char> = Vec::new();                            // A vector to hold the cargo in transit
        for item in cargo_stack[from].drain(start..stack_length) {  // Get the desired range of items out of the stack
            moving_cargo.push(item);                                            // Push them onto the moving stack
        }
        cargo_stack[to].append(&mut moving_cargo);                              // Add the moving stack back onto the correct cargo stack.
    }

    // Find the top ones so we can report it back to the elves
    let mut top_crates: String = String::new();
    for stack in cargo_stack {
        // Get the top item from the stacks and add them to the top_crates string.
        top_crates.push(*stack.get(stack.len()-1).unwrap());
    }

    // Print the output to the user.
    println!("With CrateMover 9001 the resulting order is: {}", top_crates);

}

fn main() {
    task_one();
    task_two();
}
