use std::path::Path;
use std::fs::{File};
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// First task points system
fn _points(opponent: char, you: char) -> u8 {
    match opponent {
        'A' => {
            match you {
                'X' => { return 1 + 3; }
                'Y' => { return 2 + 6; }
                'Z' => { return 3 + 0; }
                _ => {}
            }
        }
        'B' => {
            match you {
                'X' => { return 1 + 0; }
                'Y' => { return 2 + 3; }
                'Z' => { return 3 + 6; }
                _ => {}
            }
        }
        'C' => {
            match you {
                'X' => { return 1 + 6; }
                'Y' => { return 2 + 0; }
                'Z' => { return 3 + 3; }
                _ => {}
            }
        }
        _ => { return 0; /* Should never happen */ }
    }

    return 0; // Should never happen
}

// Second task points system
fn points_part_2(opponent: char, you: char) -> u8 {
    const OPP_ROCK: char = 'A';
    const OPP_PAPER: char = 'B';
    const OPP_SCISSORS: char = 'C';

    const VICT_POINTS: u8 = 6;
    const DRAW_POINTS: u8 = 3;
    const LOSS_POINTS: u8 = 0;

    const YOU_ROCK: u8 = 1;
    const YOU_PAPER: u8 = 2;
    const YOU_SCISSORS: u8 = 3;

    match opponent {
        OPP_ROCK => {
            match you {
                'X' => { return LOSS_POINTS + YOU_SCISSORS; }
                'Y' => { return DRAW_POINTS + YOU_ROCK; }
                'Z' => { return VICT_POINTS + YOU_PAPER; }
                _ => {}
            }
        }
        OPP_PAPER => {
            match you {
                'X' => { return LOSS_POINTS + YOU_ROCK; }
                'Y' => { return DRAW_POINTS + YOU_PAPER; }
                'Z' => { return VICT_POINTS + YOU_SCISSORS; }
                _ => {}
            }
        }
        OPP_SCISSORS => {
            match you {
                'X' => { return LOSS_POINTS + YOU_PAPER; }
                'Y' => { return DRAW_POINTS + YOU_SCISSORS; }
                'Z' => { return VICT_POINTS + YOU_ROCK; }
                _ => {}
            }
        }
        _ => { return 0; /* Should never happen */ }
    }

    return 0; // Should never happen
}

fn main() {
    let input_path = Path::new("input");
    let _example_path = Path::new("example");
    let mut total_score: u64 = 0;

    // Read all the lines of the file
    if let Ok(lines) = read_lines(input_path) {

        // Go throug each of the lines
        for line in lines {

            // Error handling
            match line {
                Ok(value) => {
                    // Variables to store the opponents and your move
                    let mut opponent: char = ' ';
                    let mut you: char = ' ';

                    // This is probably as very inneficient way to do this, but it works ¯\_(ツ)_/¯
                    for (i, char) in value.chars().enumerate() {
                        match i {
                            // Store the first char as the opponents move, and the third char as your move.
                            0 => {opponent = char;}
                            2 => {you = char;}
                            _ => {}
                        }
                    }
                    let points = points_part_2(opponent, you);  // Get the points
                    total_score += points as u64;                   // Add them to the sum of points
                    //println!("{}, {}, {}", opponent, you, points);
                }
                Err(_) => {}
            }
        }
    }

    println!("The total score is: {}", total_score);
}
