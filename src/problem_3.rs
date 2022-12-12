use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

fn split_and_find_common_letters(s: &str) -> char {
    let split_point = s.len() / 2;
    let (first_half, second_half) = s.split_at(split_point);

    let second: HashSet<char> = second_half.chars().collect();

    let intersection = first_half
        .chars()
        .filter(|c| second.contains(&c))
        .collect::<Vec<char>>();

    return *intersection.first().unwrap();
}

fn find_common_letter(first: &String, second: &String, third: &String) -> char {
    let second_set: HashSet<char> = second.chars().collect();
    let third_set: HashSet<char> = third.chars().collect();

    let intersection = first
        .chars()
        .filter(|c| second_set.contains(&c) && third_set.contains(&c))
        .collect::<Vec<char>>();

    return *intersection.first().unwrap();
}

fn score_letter(c: char) -> u32 {
    let mut map = HashMap::new();

    // Iterate over the letters 'a' through 'z'
    for ch in 'a'..='z' {
        // Insert each letter as a key and its corresponding number as the value into the HashMap
        map.insert(ch, ch as u8 - 'a' as u8 + 1);
    }

    for ch in 'A'..='Z' {
        map.insert(ch, ch as u8 - 'A' as u8 + 27);
    }

    return *map.get(&c).expect("Missing value ${common_letter}!") as u32;
}

pub fn problem_3() -> u32 {
    let text = String::from(include_str!("problem_3.txt"));
    let buffer = BufReader::new(text.as_bytes());
    let mut scores: Vec<u32> = Vec::new();

    for line in buffer.lines() {
        let common_letter = split_and_find_common_letters(&line.unwrap());
        scores.push(score_letter(common_letter));
    }

    let total: u32 = scores.iter().sum();

    return total;
}

pub fn problem_3_part_2() -> u32 {
    let text = String::from(include_str!("problem_3.txt"));
    let vec: Vec<String> = text.split_whitespace().map(str::to_string).collect();

    let scores = vec
        .chunks(3)
        .map(|chunk| score_letter(find_common_letter(&chunk[0], &chunk[1], &chunk[2])));
    
    let total: u32 = scores.sum();

    return total;
}
