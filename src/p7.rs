use std::{fs, collections::HashMap, cmp::Ordering};

#[derive(Debug)]
struct Hand {
    bid: u32,
    card_type: i8,
    values: [i8; 5]
}

impl Hand {
    fn new() -> Self {
        Hand {
            bid: 0,
            card_type: 0,
            values: [0; 5]
        }
    }

    fn calculate_type(&mut self, consider_joker: bool) {
        let mut max = 0;
        let mut counter: HashMap<i8, i8> = HashMap::new();
        for val in &self.values {
            let count = counter.entry(*val).or_insert(0);
            *count += 1;
            if *count > max {
                max = *count;
            }
        }
        let joker: i8;
        if consider_joker {
            let joker_val = 1;
            joker = counter.get(&joker_val).copied().unwrap_or(0);
        } else {
            joker = 0;
        }
        if counter.len() == 5 {
            self.card_type = if joker == 0 {1} else {2} ;
        } else if counter.len() == 4 {
            self.card_type = if joker > 0 {4} else {2};
        } else if counter.len() == 3 {
            if joker > 0 && max == 2 {
                self.card_type = if joker == 2 {6} else {5};
            } else if joker > 0 && max == 3 {
                self.card_type = 6;
            } else {
                self.card_type = if max == 2 {3} else {4};
            }
        } else if counter.len() == 2 {
            if joker > 0 {
                self.card_type = 7;
            } else {
                self.card_type = if max == 3 {5} else {6};
            }
        } else {
            self.card_type = 7;
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self.card_type > other.card_type {
            return Ordering::Greater
        }
        if self.card_type < other.card_type {
            return Ordering::Less
        }

        for i in 0..self.values.len() {
            if self.values[i] > other.values[i] {
                return Ordering::Greater
            }
            if self.values[i] < other.values[i] {
                return Ordering::Less
            }
        }

        Ordering::Equal
    }
}

pub fn solve() {
    let input_result = fs::read_to_string("inputs/p7.txt");
    let input = match input_result {
        Ok(i) => i,
        Err(_) => return
    };
    puzzle(&input, false);
    puzzle(&input, true)
}

fn puzzle(input: &String, consider_joker: bool) {
    let mut digit = 0;
    let mut index = 0;
    let mut seen_space = false;
    let mut hands: Vec<Hand> = Vec::new();
    let mut hand = Hand::new();
    let label_map = get_label_map(consider_joker);
    for c in input.chars() {
        if c == '\n' {
            hand.bid = digit;
            hand.calculate_type(consider_joker);
            hands.push(hand);
            hand = Hand::new();
            digit = 0;
            seen_space = false;
        } else if c == ' ' {
            index = 0;
            seen_space = true;
        } else if !seen_space {
            hand.values[index] = label_map.get(&c).copied().unwrap();
            index += 1;
        } else { 
            digit = digit*10 + c.to_digit(10).unwrap();
        }
    }
    hand.bid = digit;
    hand.calculate_type(consider_joker);
    hands.push(hand);
    hands.sort_by(|a, b| a.cmp(b));
    let mut soln = 0;
    let mut index = 1;
    for h in hands {
        soln += h.bid as usize * index;
        index += 1;
    }
    println!("Puzzle: {soln}")
}


fn get_label_map(consider_joker: bool) -> HashMap<char, i8> {
    let joker_val: i8 = if consider_joker {1} else {11};
    HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
        ('J', joker_val)
    ])
}
