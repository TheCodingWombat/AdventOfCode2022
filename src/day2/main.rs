use std::fs;

fn main() {

    let input: String = fs::read_to_string("input.txt").unwrap();
    let lines_iter = input.lines().collect::<Vec<_>>();

    let mut split_lines = lines_iter.iter().map(|line| line.split_whitespace());

    let sum = split_lines.map(|mut line_iter| get_score(line_iter.next().unwrap(), line_iter.next().unwrap())).sum::<usize>();

    dbg!("Sum: {:?}", sum);

}

fn get_score(a: &str, b: &str) -> usize {
    let theirs = a.chars().next().unwrap() as usize - 65;
    let yours = b.chars().next().unwrap() as usize - 88;

    // println!("{:?}, {:?}", theirs, yours);
    let c = (theirs + 3 - yours) % 3;
    (match c {
        0 => 3,
        1 => 0,
        2 => 6,
        _ => panic!("cannot happen")
    }) + 1 + yours
}
