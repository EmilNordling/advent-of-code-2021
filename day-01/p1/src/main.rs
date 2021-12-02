fn main() {
    let x = include_str!("../../input.txt")
            .lines()
            .map(|l| l.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
            .windows(2)
            .filter(|x| x[0] < x[1])
            .count();

    print!("{}", x);
}
