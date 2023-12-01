use std::fs;

fn main() {
    let input_result = fs::read_to_string("input.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return
    };
    first_soln(&input);
    second_soln(&input);
}

fn first_soln(input: &str) {
    let mut sum = 0;
    let mut first = 0;
    let mut second = 0;
    for c in (input).chars() {
        if c.is_digit(10) {
            if first == 0 {
                first = c.to_digit(10).unwrap();
            } else {
                second = c.to_digit(10).unwrap();
            }
        } else if c == '\n' {
            first = if second == 0 {first*10 + first} else {first*10 + second};
            sum += first;
            first = 0;
            second = 0;
        }
    }
    first = if second == 0 {first*10 + first} else {first*10 + second};
    sum += first;
    println!("First puzzle sum is {sum}");
}

fn second_soln(input: &str) {
    let mut sum = 0;
    let mut first = 0;
    let mut second = 0;
    let mut start = 0;
    let mut num = String::new();
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
    let mut has_match: bool;
    for c in (input).chars() {
        if c.is_digit(10) {
            if first == 0 {
                first = c.to_digit(10).unwrap() as usize;
            } else {
                second = c.to_digit(10).unwrap() as usize;
            }
        } else if c == '\n' {
            first = if second == 0 {first*10 + first} else {first*10 + second};
            sum += first;
            first = 0;
            second = 0;
            num.clear();
            start = 0;
        } else {
            num.push(c);
            has_match = false;
            while !has_match && start < num.len() {
                let size = num.len() - start;
                let mut index = 0;
                while index < numbers.len() {
                    if size > numbers[index].len() {
                        index += 1;
                        continue;
                    }
                    if numbers[index] == &num[start..] {
                        if first == 0 {
                            first = index + 1;
                        } else {
                            second = index + 1;
                        }
                        start += 1;
                        has_match = true;
                        break;
                    } else if numbers[index].starts_with(&num[start..]) {
                        has_match = true;
                        break;
                    }
                    index += 1;
                }
                if !has_match  {
                    start += 1;
                }
            }
            if start == num.len() {
                num.clear();
                start = 0;
            }
        }
    }
    first = if second == 0 {first*10 + first} else {first*10 + second};
    sum += first;
    println!("Second puzzle sum is {sum}");
}
