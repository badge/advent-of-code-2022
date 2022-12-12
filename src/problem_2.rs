use std::collections::HashMap;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Target {
    Win,
    Lose,
    Draw
}


fn read_shapes(reader: BufReader<impl BufRead>, shape_map: &HashMap<&str, Shape>, target_map: &HashMap<&str, Target>) -> Vec<(Shape, Target)> {
    // Iterate over the lines of the BufReader
    let mut shapes: Vec<(Shape, Target)> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        // Split the line into two letters
        let mut letters = line.split(" ");
        let letter1 = letters.next().expect("Failed to parse first letter");
        let letter2 = letters.next().expect("Failed to parse second letter");

        // Look up the Shape instances associated with the letters
        let shape = shape_map.get(letter1).expect("Failed to find shape for first letter");
        let target = target_map.get(letter2).expect("Failed to find shape for second letter");

        // Add the Shape instances to the list
        shapes.push((*shape, *target));
    }

    // Return the list of Shape instances
    return shapes

}

fn score_round(opponent: &Shape, us: &Shape) -> i32 {

    let base_score_map = HashMap::from([
        (Shape::Rock, 1),
        (Shape::Paper, 2),
        (Shape::Scissors, 3),
    ]);

    let score_map = HashMap::from([
        (
            Shape::Rock,
            HashMap::from([
                (Shape::Rock, 3),
                (Shape::Paper, 6),
                (Shape::Scissors, 0),
            ])
        ),
        (
            Shape::Paper,
            HashMap::from([
                (Shape::Rock, 0),
                (Shape::Paper, 3),
                (Shape::Scissors, 6),
            ])
        ),
        (
            Shape::Scissors,
            HashMap::from([
                (Shape::Rock, 6),
                (Shape::Paper, 0),
                (Shape::Scissors, 3),
            ])
        ),
    ]);

    let base_score = base_score_map.get(us).expect("Missing value!");
    let score = score_map.get(opponent).expect("Missing value!").get(us).expect("Missing value!");
    return base_score + score
}

/// Get the shape we need to play based on the opponent's shape
/// and the target outcome (win, lose, or draw)
fn get_our_shape(shape: &Shape, target: &Target) -> Shape {
    let shape_target_map = HashMap::from([
        (
            Shape::Rock,
            HashMap::from([
                (Target::Win, Shape::Paper),
                (Target::Lose, Shape::Scissors),
            ])
        ),
        (
            Shape::Paper,
            HashMap::from([
                (Target::Win, Shape::Scissors),
                (Target::Lose, Shape::Rock),
            ])
        ),
        (
            Shape::Scissors,
            HashMap::from([
                (Target::Win, Shape::Rock),
                (Target::Lose, Shape::Paper),
            ])
        ),
    ]);

    if *target == Target::Draw {
        return shape.clone();
    }

    let target_map = shape_target_map.get(shape).expect("Blah!");
    return target_map.get(target).expect("Basdf!").clone();



}

fn get_scores(rounds: Vec<(Shape, Target)>) -> Vec<i32> {
    // Iterate over the lists and compute the sum of each list
    let mut scores: Vec<i32> = Vec::new();
    for round in rounds {
        let our_shape = get_our_shape(&round.0, &round.1);
        let score: i32 = score_round(&round.0, &our_shape);
        scores.push(score);
    }
    return scores
}

pub fn problem_2() -> i32 {

    let shape_map = HashMap::from([
        ("A", Shape::Rock),
        ("B", Shape::Paper),
        ("C", Shape::Scissors),
    ]);

    let target_map = HashMap::from([
        ("X", Target::Lose),
        ("Y", Target::Draw),
        ("Z", Target::Win),
    ]);

    let text = String::from(include_str!("problem_2.txt"));
    let buffer = BufReader::new(text.as_bytes());
    
    let shapes = read_shapes(buffer, &shape_map, &target_map);
    let scores = get_scores(shapes);

    return scores.iter().sum();
}
