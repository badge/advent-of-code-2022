use std::io::{BufRead, BufReader};

// Reads a file, splits it into blocks, and parses the numbers in each block
fn read_numbers_from_file(buffer: BufReader<impl BufRead>) -> Vec<Vec<i32>> {
    // Open the file

    // Read the file line by line
    let mut blocks: Vec<Vec<i32>> = Vec::new();
    let mut current_block: Vec<i32> = Vec::new();
    for line in buffer.lines() {
        let line = line.expect("Failed to read line");

        // If the line is empty, start a new block
        if line.is_empty() {
            blocks.push(current_block);
            current_block = Vec::new();
            continue;
        }

        // Otherwise, parse the number on the line and add it to the current block
        let number = line.parse::<i32>().expect("Failed to parse number");
        current_block.push(number);
    }

    // Return the blocks
    blocks
}

fn get_sums(lists: Vec<Vec<i32>>) -> Vec<i32> {
    // Iterate over the lists and compute the sum of each list
    let mut sums: Vec<i32> = Vec::new();
    for list in lists {
        let sum: i32 = list.iter().sum();
        sums.push(sum);
    }
    return sums
}


fn largest_sum(sums: Vec<i32>) -> i32 {

    // Return the largest sum
    *sums.iter().max().unwrap()
}

fn three_largest(sums: Vec<i32>) -> i32 {
    let mut vec = Vec::from(sums);

    vec.sort_by(|a, b| b.cmp(a));

    let largest_three: Vec<i32> = vec.into_iter().take(3).collect();

    // Compute the sum of the three largest values
    let sum: i32 = largest_three.iter().sum();
    return sum
}

pub fn problem_1() -> i32 {

    let text = String::from(include_str!("problem_1.txt"));
    let buffer = BufReader::new(text.as_bytes());
    let list_of_lists = read_numbers_from_file(buffer);
    let sums = get_sums(list_of_lists);

    let max_calories = largest_sum(sums);

    return max_calories;
}

pub fn problem_1_part_2() -> i32 {

    let text = String::from(include_str!("problem_1.txt"));
    let buffer = BufReader::new(text.as_bytes());
    let list_of_lists = read_numbers_from_file(buffer);
    let sums = get_sums(list_of_lists);

    return three_largest(sums);

}
