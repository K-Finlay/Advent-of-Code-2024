use input::get_input;

mod input;

pub fn run() {
    println!("--- Day 04 ---");

    solution_part_one();
    solution_part_two();

    println!();
}

fn solution_part_one() {
    const WORD: &str = "XMAS";

    let input = get_input(); // Assume this returns Vec<Vec<char>> representing the grid
    let mut count = 0;

    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
        (1, -1),  // Down-Left
    ];

    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    for row in 0..rows {
        for col in 0..cols {
            for (dx, dy) in directions {
                let mut matched = true;

                for (k, ch) in WORD.chars().enumerate() {
                    let new_row = row + dx * k as i32;
                    let new_col = col + dy * k as i32;

                    if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
                        matched = false;
                        break;
                    }

                    if input[new_row as usize][new_col as usize] != ch {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    count += 1;
                }
            }
        }
    }

    println!("Pt1 - Word search XMAS count: {count}");
}

fn solution_part_two() {
    let input = get_input(); // Assume this returns Vec<Vec<char>> representing the grid
    let mut count = 0;

    let rows = input.len();
    let cols = input[0].len();

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if input[row][col] != 'A' {
                continue;
            }

            let c0 = input[row - 1][col - 1]; // Up-Left
            let c1 = input[row + 1][col + 1]; // Down-Right
            let c2 = input[row + 1][col - 1]; // Down-Left
            let c3 = input[row - 1][col + 1]; // Up-Right

            if [c0, c1, c2, c3].iter().all(|&c| c == 'M' || c == 'S') && c0 != c1 && c2 != c3 {
                count += 1;
            }
        }
    }

    println!("Pt2 - Word search X-MAS count: {count}");
}
