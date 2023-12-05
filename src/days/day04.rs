use std::fs;

#[derive(Debug)]
struct Number {
    raw: [char; 2],
}

pub fn solution() {
    let file: String = fs::read_to_string("input/day4").expect("Unable to open the file");

    // Split into hands
    let hands: Vec<&str> = file.split("\n").collect();

    let mut winning_numbers: Vec<Vec<i32>> = Vec::new();
    let mut your_hands: Vec<Vec<i32>> = Vec::new();

    for hand in hands {
        let mut winning_row: Vec<i32> = Vec::new();
        let mut your_hand_row: Vec<i32> = Vec::new();

        // Split the "Card N: " from the rest of the hand
        let hand_parts: Vec<&str> = hand.split(": ").collect();
        if hand_parts.len() < 2 {
            continue;
        }
        let hand: &str = hand_parts[1];

        // Split the winning numbers from the numbers you have
        let hand_parts: Vec<&str> = hand.split(" | ").collect();

        for (i, hand_part) in hand_parts.iter().enumerate() {
            let mut step: usize = 1;
            let mut current_number: Option<Number> = None;
            for char in hand_part.chars() {
                match step {
                    1 => {
                        if char.is_digit(10) {
                            current_number = Some(Number { raw: [char, ' '] });
                            step = 2;
                        }
                    },
                    2 => {
                        if char.is_digit(10) {
                            let mut number = current_number.unwrap();
                            number.raw[1] = char;
                            let actual_number = number.raw.iter().collect::<String>().trim().parse::<i32>().unwrap();
                            if i == 0 {
                                winning_row.push(actual_number);
                            } else {
                                your_hand_row.push(actual_number);
                            }
                            current_number = None;
                            step = 1; // Reset step to 1 after processing a number
                        } else {
                            let actual_number = current_number.unwrap().raw[0].to_digit(10).unwrap() as i32;
                            if i == 0 {
                                winning_row.push(actual_number);
                            } else {
                                your_hand_row.push(actual_number);
                            }
                            current_number = None;
                            step = 1; // Reset step to 1 after processing a number
                        }
                    },
                    3 => step = 1, // skip empty space
                    _ => panic!("Shouldn't even be seeing this"),
                };
            }
            // Handle the last number if it's a single digit
            if let Some(number) = current_number {
                let actual_number = number.raw[0].to_digit(10).unwrap() as i32;
                if i == 0 {
                    winning_row.push(actual_number);
                } else {
                    your_hand_row.push(actual_number);
                }
            }
        }
        winning_numbers.push(winning_row);
        your_hands.push(your_hand_row);
    }

    let mut total_points: i32 = 0;

    for i in 0..winning_numbers.len() {
        let mut points: i32 = 0;
        for &number in &your_hands[i] {
            if winning_numbers[i].contains(&number) {
                points = if points == 0 { 1 } else { points * 2 };
            }
        }
        total_points += points;
    }

    println!("Total points: {}", total_points);
}

/*

Picking one up, it looks like each card has two lists of numbers separated by a vertical bar (|): a list of winning numbers and then a list of numbers you have.
You organize the information into a table (your puzzle input).

As far as the Elf has been able to figure out, you have to figure out which of the numbers you have appear in the list of winning numbers. 
The first match makes the card worth one point and each match after the first doubles the point value of that card.

For example:

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

In the above example, card 1 has five winning numbers (41, 48, 83, 86, and 17) and eight numbers you have (83, 86, 6, 31, 17, 9, 48, and 53). 
Of the numbers you have, four of them (48, 83, 17, and 86) are winning numbers! 
That means card 1 is worth 8 points (1 for the first match, then doubled three times for each of the three matches after the first).

    Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
    Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
    Card 4 has one winning number (84), so it is worth 1 point.
    Card 5 has no winning numbers, so it is worth no points.
    Card 6 has no winning numbers, so it is worth no points.

So, in this example, the Elf's pile of scratchcards is worth 13 points.

Take a seat in the large pile of colorful cards. How many points are they worth in total?

---

Notes:

You have a card sheet that contains cards.
Cards contain:
- card #
- winning numbers
- played numbers (a.k.a; your numbers)

- numbers are two characters
    - if the first character is blank, then it is a single digit number

*/