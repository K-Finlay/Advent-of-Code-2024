use input::get_input;

mod input;

pub fn run() {
    println!("--- Day 01 ---");

    solution_part_one();
    solution_part_two();

    println!();
}

fn solution_part_one() {
    let (mut lhs, mut rhs) = get_input();

    lhs.sort();
    rhs.sort();

    let res: i32 = lhs.iter().zip(rhs.iter()).map(|(a, b)| (a - b).abs()).sum();
    println!("Pt1 - Total distance: {res}");
}

fn solution_part_two() {
    let (mut lhs, mut rhs) = get_input();

    lhs.sort();
    rhs.sort();

    let mut score = 0;
    for i in lhs {
        score += i * rhs.iter().filter(|x| **x == i).count() as i32;
    }

    println!("Pt2 - Similarity score: {score}");
}
