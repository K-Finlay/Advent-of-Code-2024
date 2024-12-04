const INPUT: &str = "7 6 4 2 1
                     1 2 7 8 9
                     9 7 6 2 1
                     1 3 2 4 5
                     8 6 4 4 1
                     1 3 6 7 9";

pub fn get_input() -> Vec<Vec<i32>> {
    INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
