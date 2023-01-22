use std::fs;

fn main() {

    let input: String = fs::read_to_string("src/input2.txt").unwrap();
    let mut lines_iter = input
        .lines()
        .collect::<Vec<_>>();
    let mut group = lines_iter.split(|line| line.is_empty());

    let max = group.map(|list| list.iter()
                                   .map(|v| v.parse::<u32>().unwrap())
                                   .sum::<u32>())
                   .max();

    println!("{:?}", max)

}
