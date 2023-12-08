use std::fs;

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p2.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return,
    };
    first_puzzle(&input);
    second_puzzle(&input);
}

fn first_puzzle(input: &str) {
    let mut sum = 0;
    let mut game_id = 0;
    let valid_counts = [12, 13, 14];
    let mut digit = 0;
    let mut valid_line = true;
    let mut prev_char_is_space = false;
    let mut index = 3;
    for c in input.chars() {
        if c == '\n' {
            sum = if valid_line { sum + game_id } else { sum };
            digit = 0;
            valid_line = true;
            continue;
        }
        if !valid_line {
            continue;
        }
        if c.is_digit(10) {
            digit = digit * 10 + c.to_digit(10).unwrap();
            continue;
        }
        match c {
            ':' => {
                game_id = digit;
                digit = 0;
            }
            ' ' => {
                prev_char_is_space = true;
            }
            'r' => {
                index = 0;
            }
            'g' => {
                index = 1;
            }
            'b' => {
                index = 2;
            }
            _ => {}
        }
        if index < 3 {
            if prev_char_is_space && digit > valid_counts[index] {
                valid_line = false;
                prev_char_is_space = false;
            }
            digit = 0;
            index = 3;
        }
    }
    sum = if valid_line { sum + game_id } else { sum };
    println!("First puzzle sum is {sum}");
}

fn second_puzzle(input: &str) {
    let mut sum = 0;
    let mut digit = 0;
    let mut seen_colon = false;
    let mut max_red = 1;
    let mut max_blue = 1;
    let mut max_green = 1;
    for c in input.chars() {
        if c == '\n' {
            sum += max_red * max_green * max_blue;
            digit = 0;
            max_red = 1;
            max_blue = 1;
            max_green = 1;
            seen_colon = false;
            continue;
        }
        if c.is_digit(10) && seen_colon {
            digit = digit * 10 + c.to_digit(10).unwrap();
            continue;
        }
        match c {
            ':' => {
                seen_colon = true;
            }
            'r' => {
                max_red = if digit > max_red { digit } else { max_red };
                digit = 0;
            }
            'g' => {
                max_green = if digit > max_green { digit } else { max_green };
                digit = 0
            }
            'b' => {
                max_blue = if digit > max_blue { digit } else { max_blue };
                digit = 0;
            }
            _ => {}
        }
    }
    sum += max_red * max_green * max_blue;
    println!("Second puzzle sum is {sum}");
}
