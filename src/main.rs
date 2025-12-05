use num_traits::Euclid;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // L => counterclockwise, numbers go down
    // R => clockwise, numbers go up
    // 2625 too low
    // 7217
    // 7915

    let mut position = 50;
    let mut count = 0;

    if let Ok(lines) = read_lines("src/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let direction = match &line[..1] {
                "R" => 1,
                "L" => -1,
                _ => 0 // fails silently here unfortch
            };

            // https://users.rust-lang.org/t/basic-problem-parsing-a-string-to-int/77769
            let magnitude = &line[1..].trim().parse::<i32>().unwrap(); 
            let vector = magnitude * direction;
            let next_position = position + vector;
            println!("count: {count}, pos: {position}, change: {vector} npos: {next_position}");

            let div = Euclid::div_euclid(&next_position, &100);

            let rem = Euclid::rem_euclid(&next_position, &100);

            if position == 0 {
                if next_position > 0 {
                    // covers landing on zero via div(100, 100) -> 1
                    count += div;
                };

                if next_position < 0 {
                    if -100 < next_position {
                        // nothing, bc we cover the starting position in other cases
                    } else if rem == 0 {
                        // this is for each time, end landing on 0
                        count += div.abs();
                    } else if next_position < -100 {
                        count += div.abs() - 1;
                    };
                }

            } else if position > 0 {
                if next_position == 0 {
                    count += 1;
                } else if next_position > 0 {
                    count += div;
                } else if next_position < 0 {
                    if -100 < next_position {
                        count += 1;
                    } else if rem == 0 {
                        count += div.abs() + 1;
                    } else if next_position < -100 {
                        count += div.abs(); 
                    };
                }
            };

            position = rem;  
            
        }

    }
    println!("Count @ 0: {count}");
    let a = Euclid::div_euclid(&50, &100);
    let b = Euclid::div_euclid(&100, &100);
    let c = Euclid::div_euclid(&150, &100);
    println!("{a}, {b}, {c}");
    let d = Euclid::div_euclid(&-50, &100);
    let e = Euclid::div_euclid(&-100, &100);
    let f = Euclid::div_euclid(&-110, &100);
    let g = Euclid::div_euclid(&-200, &100);
    println!("{d}, {e}, {f}, {g}");
}

// rust by example
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
