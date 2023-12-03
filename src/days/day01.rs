use std::fs;

#[allow(dead_code)]
pub fn solution() {
    let file = fs::read_to_string("input/day1").expect("Unable to open the file");

    // brute force lol
    let trimmed_file = file.replace("one", "o1ne")
        .replace("two", "t2wo")
        .replace("three", "th3ree")
        .replace("four", "fo4ur")
        .replace("five", "fi5ve")
        .replace("six", "si6x")
        .replace("seven", "sev7en")
        .replace("eight", "ei8ght")
        .replace("nine", "ni9ne");

    let items = trimmed_file.split("\n");

    let mut sum_values: Vec<i32> = Vec::new();

    for (i, item) in items.enumerate() {
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
        println!("{}) {} - {}{}", i+1, item, first, second);
        sum_values.push((first * 10) + second);
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

---

Part 2:

Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?

---

Notes:
- We could possibly just scan the entire document and replace the words with their digit equivalent before letting it run?
- This may not be a good idea as some of the worded numbers overlap and we need to maintain their integrity.
- Replacing them and putting the number before or after could ruin that. Placing the number in the middle might be the easiest win.

*/