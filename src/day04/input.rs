pub const INPUT: &str = "MMMSXXMASM
                         MSAMXMSMSA
                         AMXSXMAAMM
                         MSAMASMSMX
                         XMASAMXAMM
                         XXAMMXXAMA
                         SMSMSASXSS
                         SAXAMASAAA
                         MAMMMXMMMM
                         MXMXAXMASX";

pub fn get_input() -> Vec<Vec<char>> {
    INPUT
        .lines()
        .map(|line| line.split_whitespace().flat_map(|s| s.chars()).collect())
        .collect()
}
