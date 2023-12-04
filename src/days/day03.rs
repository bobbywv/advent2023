use std::fs;

#[derive(Debug)]
struct Schematic {
    block_size: usize,
    raw: Vec<char>,
}

impl Schematic {
    fn new(input: String, delimiter: &str) -> Schematic {
        // find the block size
        let index = input.find(delimiter);
        let block_size: usize = match index {
            Some(i) => i,
            None => panic!("Invalid delimiter - Not found in input"),
        };

        Schematic { 
            raw: input.replace(delimiter, "").chars().collect(),
            block_size
        }
    }

    fn calculate_neighbors(&self, position: usize) -> Vec<usize> {
        let row = position / self.block_size;
        let col = position % self.block_size;
        let mut neighbors = Vec::new();

        for dr in [-1, 0, 1].iter() {
            for dc in [-1, 0, 1].iter() {
                if *dr == 0 && *dc == 0 {
                    continue;
                }
                let new_row = (row as i32) + dr;
                let new_col = (col as i32) + dc;
                if new_row >= 0 && new_row < self.block_size as i32 && new_col >= 0 && new_col < self.block_size as i32 {
                    neighbors.push((new_row as usize) * self.block_size + (new_col as usize));
                }
            }
        }
        neighbors
    }
}

// Convert a slice of i32 ints into a single digit that the array represents.
// Does this function already exist? If so, I couldn't find it.
// This doesn't do any bounds checking because the input doesn't seem to have very long ints in it.
// WILL PANIC ON EVEN MODERATELY SIZED INTS
pub fn shortvec2int(input: &[i32]) -> i32 {
    let mut output: i32 = 0;
    for &digit in input {
        output = output * 10 + digit;
    }
    output
}

pub fn solution() {
    let file: String = fs::read_to_string("input/day3").expect("Unable to open the file");
    let schematic: Schematic = Schematic::new(file, "\n");

    let mut current_number: Vec<i32> = Vec::new();
    let mut current_positions: Vec<usize> = Vec::new();
    let mut symbol_positions: Vec<usize> = Vec::new();
    let mut parts: Vec<(i32, Vec<usize>)> = Vec::new();

    for (i, char) in schematic.raw.iter().enumerate() {
        match char {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                current_number.push(char.to_digit(10).unwrap() as i32);
                current_positions.push(i);
            },
            '.' => {
                if !current_number.is_empty() {
                    parts.push((shortvec2int(&current_number), current_positions.clone()));
                    current_number.clear();
                    current_positions.clear();
                }
            },
            _ => {
                if !current_number.is_empty() {
                    parts.push((shortvec2int(&current_number), current_positions.clone()));
                    current_number.clear();
                    current_positions.clear();
                }
                symbol_positions.push(i);
            },
        };
    }

    // Add the last number if it wasn't added already
    if !current_number.is_empty() {
        parts.push((shortvec2int(&current_number), current_positions));
    }

    let mut sum = 0;

    for (number, positions) in &parts {
        for &position in positions {
            let neighbors = schematic.calculate_neighbors(position);
            if neighbors.into_iter().any(|neighbor| symbol_positions.contains(&neighbor)) {
                sum += number;
                break;
            }
        }
    }

    println!("{}", sum);

}

/*

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. 
If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. 
There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, 
even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..


In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). 
Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

---

Notes:
- There are instances of single digit numbers, so the tracker needs to accomodate for this
- Each line is 140 characters long consistently, so we can use this to our advantage.

*/