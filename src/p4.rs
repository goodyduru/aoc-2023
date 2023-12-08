use std::fs;

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p4.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return,
    };
    first_puzzle(&input);
    second_puzzle(&input);
}

fn first_puzzle(input: &String) {
    let mut winning_cards: [u32; 10] = [0; 10];
    let mut my_cards: [u32; 25] = [0; 25];
    let mut index = 0;
    let mut digit: u32 = 0;
    let mut seen_colon = false;
    let mut seen_bar = false;
    let mut sum = 0;
    for c in input.chars() {
        if c == ':' {
            seen_colon = true;
            continue;
        }
        if !seen_colon {
            continue;
        }
        if c.is_digit(10) {
            digit = digit * 10 + c.to_digit(10).unwrap();
            continue;
        }
        if c == ' ' && digit > 0 {
            if seen_bar {
                my_cards[index] = digit;
            } else {
                winning_cards[index] = digit;
            }
            index += 1;
            digit = 0;
            continue;
        }
        match c {
            ':' => {
                seen_colon = true;
            }
            '|' => {
                seen_bar = true;
                index = 0;
            }
            '\n' => {
                my_cards[index] = digit;
                winning_cards.sort();
                my_cards.sort();
                sum += sum_cards(&winning_cards, &my_cards);
                index = 0;
                seen_colon = false;
                seen_bar = false;
                digit = 0;
            }
            _ => {
                continue;
            }
        }
    }
    winning_cards.sort();
    my_cards.sort();
    sum += sum_cards(&winning_cards, &my_cards);
    println!("First puzzle sum is {sum}");
}

fn sum_cards(winning_cards: &[u32], my_cards: &[u32]) -> i32 {
    let mut j = 0;
    let mut points = 0;
    for s in winning_cards {
        let result = my_cards[j..].binary_search(s);
        match result {
            Ok(k) => {
                points = if points == 0 { 1 } else { points * 2 };
                j = k + 1;
            }
            Err(k) => {
                j = k;
            }
        }
    }
    points
}

fn second_puzzle(input: &String) {
    const MAX_WINNINGS: usize = 10;
    let mut winning_cards: [u32; MAX_WINNINGS] = [0; MAX_WINNINGS];
    let mut my_cards: [u32; 25] = [0; 25];
    let mut copies: [u32; MAX_WINNINGS] = [1; MAX_WINNINGS];
    let mut line_num = 0;
    let mut points: usize;
    let mut copy: u32;
    let mut index = 0;
    let mut digit: u32 = 0;
    let mut seen_colon = false;
    let mut seen_bar = false;
    let mut sum = 0;
    for c in input.chars() {
        if c == ':' {
            seen_colon = true;
            continue;
        }
        if !seen_colon {
            continue;
        }
        if c.is_digit(10) {
            digit = digit * 10 + c.to_digit(10).unwrap();
            continue;
        }
        if c == ' ' && digit > 0 {
            if seen_bar {
                my_cards[index] = digit;
            } else {
                winning_cards[index] = digit;
            }
            index += 1;
            digit = 0;
            continue;
        }
        match c {
            ':' => {
                seen_colon = true;
            }
            '|' => {
                seen_bar = true;
                index = 0;
            }
            '\n' => {
                my_cards[index] = digit;
                winning_cards.sort();
                my_cards.sort();
                copy = copies[line_num % MAX_WINNINGS];
                copies[line_num % MAX_WINNINGS] = 1;
                points = count_points(&winning_cards, &my_cards);
                for i in 1..points + 1 {
                    copies[(line_num + i) % MAX_WINNINGS] += copy;
                }
                sum += copy;
                index = 0;
                seen_colon = false;
                seen_bar = false;
                digit = 0;
                line_num += 1;
            }
            _ => {
                continue;
            }
        }
    }
    sum += copies[line_num % MAX_WINNINGS];
    println!("Second puzzle sum is {sum}");
}

fn count_points(winning_cards: &[u32], my_cards: &[u32]) -> usize {
    let mut j = 0;
    let mut points = 0;
    for s in winning_cards {
        let result = my_cards[j..].binary_search(s);
        match result {
            Ok(k) => {
                points += 1;
                j = k + 1;
            }
            Err(k) => {
                j = k;
            }
        }
    }
    points
}
