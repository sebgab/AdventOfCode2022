use std::path::Path;
use std::fs::{File};
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let input_path = Path::new("input");
    let mut elves: Vec<Vec<i64>> = Vec::new();

    // Read all the lines of the file
    if let Ok(lines) = read_lines(input_path) {

        // Have a vec to store the calorie values
        let mut calories: Vec<i64> = Vec::new();

        // Go throug each of the lines
        for line in lines {

            // Error handling
            match line {
                Ok(value) => {
                    if value == "".to_string() {
                        // If the value is empty we push the calorie vec into the list of elves and empty it out
                        elves.push(calories.clone());
                        calories.clear();
                    } else {
                        // If it is not we have a new calorie value, which we add to the calorie vec
                        let val: i64 = value.parse::<i64>().unwrap();
                        calories.push(val);
                    }
                }
                Err(_) => {}
            }
        }
    }

    // My soloution to part 1
    /*
    let mut elf_with_most_food: (i64, i64) = (-1, -1); // (elf nr, calorie nr)

    for (elf_nr, elf_calories) in elves.iter().enumerate() {
        let sum = elf_calories.iter().sum::<i64>();
        if sum > elf_with_most_food.1 {
            elf_with_most_food.0 = elf_nr as i64;
            elf_with_most_food.1 = sum;
        }
    }

    println!("The elf with the most food is elf nr {}, that elf has {} calories.", elf_with_most_food.0+1, elf_with_most_food.1);
    */

    // My soloution to part 2
    let mut elves_and_their_calories: Vec<(i64, i64)> = Vec::new(); // (elf nr, calorie nr)

    // Add them all to the vec of elves and their calories
    for (elf_nr, elf_calories) in elves.iter().enumerate() {
        let sum = elf_calories.iter().sum::<i64>();
        elves_and_their_calories.push((elf_nr as i64, sum));
    }

    // Sort the vector in a decreasing order
    elves_and_their_calories.sort_by(|a, b| b.1.cmp(&a.1));

    // Make variables to store the info we are interested in
    let mut top_three_evles_calories_summed: i64 = 0;
    let mut top_three_elves_nr: [i64; 3] = [-1, -1, -1];
    for i in 0..3 {
        // Go through the top 3 elves and use the data accordingly
        top_three_elves_nr[i] = elves_and_their_calories[i].0;
        top_three_evles_calories_summed += elves_and_their_calories[i].1;
    }

    // Print the output nicely for our end user
    println!("The top three elves are {:?}, and in total they have {} calories.", top_three_elves_nr, top_three_evles_calories_summed);

}
