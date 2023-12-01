use std::fs;

pub fn solution() {
    let file = fs::read_to_string("input/day1").expect("Unable to open the file");

    let items = file.split("\n");

    let mut sum_values: Vec<i32> = Vec::new();

    for item in items {
        let first: i32;
        let second: i32;

        let mut numbers: Vec<i32> = Vec::new();

        for ch in item.chars() {
            match ch {
                '1' => numbers.push(1),
                '2' => numbers.push(2),
                '3' => numbers.push(3),
                '4' => numbers.push(4),
                '5' => numbers.push(5),
                '6' => numbers.push(6),
                '7' => numbers.push(7),
                '8' => numbers.push(8),
                '9' => numbers.push(9),
                _ => {},
            };
        }

        first = *numbers.first().unwrap();
        second = *numbers.last().unwrap();

        sum_values.push(first * 10 + second);
    }

    let mut sum: i32 = 0;
    for num in sum_values {
        sum += num;
    }

    println!("{}", sum);

}

/*

Problem definition:

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover.
On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

---

Notes:
- The last line shows a single digit, so that means we double the digit to be first and last.
- I *think* the input has a number in each line. Not sure, but at a cursory glance this seems to be true.
- There are no zero (0) characters in the input, so we don't have to worry about that.

Tasks:
1. Read in the input
2. Separate the lines for parsing
3. Parse the line to get our first and second digit:
    3a. Scan for the first digit
    3b. Scan for the second digit (this may not exist)
        3b.1. If it doesn't exist, use the first digit as the second digit as well
4. Combine the two digits into a single number
5. Add that digit to a list
6. Iterate and sum the list = Result

*/