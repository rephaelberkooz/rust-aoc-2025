use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // L => counterclockwise, numbers go down
    // R => clockwise, numbers go up

    let mut position = 50;
    let mut count = 0;

    if let Ok(lines) = read_lines("src/input.txt") {
        println!("Entered read_lines");
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let direction = match &line[..1] {
                "R" => 1,
                "L" => -1,
                _ => 0 // fails silently here unfortch
            };

            // https://users.rust-lang.org/t/basic-problem-parsing-a-string-to-int/77769
            let magnitude = &line[1..].trim().parse::<i32>().unwrap();
            position += magnitude * direction;
            position = position % 100;
            if position < 0 {
                position += 100;
            }
            if position == 0 {
                count += 1;
            }
        }
    }
    println!("Count @ 0: {count}")
}

// rust by example
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
