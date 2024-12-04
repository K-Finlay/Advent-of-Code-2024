use input::get_input;

mod input;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Increasing,
    Decreasing,
}

pub fn run() {
    println!("--- Day 02 ---");

    let input = get_input();

    solution_part_one(&input);
    solution_part_two(&input);

    println!();
}

fn solution_part_one(input: &Vec<Vec<i32>>) {
    let mut safe = 0;
    for row in input {
        if check_safe(row).0 {
            safe += 1;
        }
    }

    println!("Pt1 - Number of safe reports: {safe}");
}

fn solution_part_two(input: &Vec<Vec<i32>>) {
    let mut safe = 0;
    for row in input {
        if check_safe(row).0 {
            safe += 1;
        } else {
            // One by one, remove each index in the array and recheck whether it is safe.
            // We are literally just brute forcing it here.
            // Is this an efficient way to do it... No.
            // Am I proud of it... Also. no.
            // Does it work... Yes!
            for index in 0..row.len() {
                let mut row = row.to_vec();
                row.remove(index);

                if check_safe(&row).0 {
                    safe += 1;
                    break;
                }
            }
        }
    }

    println!("Pt2 - Number of safe reports: {safe}");
}

fn check_safe(input: &[i32]) -> (bool, usize, usize) {
    let mut row_direction = None;
    for (index, window) in input.windows(2).enumerate() {
        let (a, b) = (window[0], window[1]);

        let direction = get_direction(a, b);
        if row_direction.is_none() {
            row_direction = Some(direction);
        }

        // Check if the direction has changed.
        // Return unsafe if true.
        if let Some(row_direction) = row_direction
            && row_direction != direction
        {
            return (false, index, index + 1);
        }

        // Check the difference between the two numbers.
        // If greater than 3 or equal, return unsafe.
        if (a - b).abs() > 3 || a == b {
            return (false, index, index + 1);
        }
    }

    (true, 0, 0)
}

fn get_direction(a: i32, b: i32) -> Direction {
    if b > a {
        Direction::Increasing
    } else {
        Direction::Decreasing
    }
}
