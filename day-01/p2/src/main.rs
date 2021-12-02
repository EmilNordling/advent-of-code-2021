fn main() {
    let x = include_str!("../../input.txt")
            .lines()
            .map(|l| l.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
            .windows(3)
            .map(|w| w.iter().sum()).collect::<Vec<u16>>()
            .windows(2)
            .filter(|w| w[0] < w[1])
            .count();

    print!("\n{}", x)
}
