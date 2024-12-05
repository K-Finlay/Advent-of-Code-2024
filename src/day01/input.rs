const INPUT: &str = "3   4
                     4   3
                     2   5
                     1   3
                     3   9
                     3   3";

pub fn get_input() -> (Vec<i32>, Vec<i32>) {
    INPUT
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip()
}
