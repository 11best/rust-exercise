fn main() {
    let round: u16 = read_input().parse().expect("can not parse to int");
    if round > 1000 {
        panic!("n is more than thousand!")
    }

    let mut min: i64;
    let mut max: i64;

    let input1: i64 = read_input().parse().expect("can not parse to int");
    let input2: i64 = read_input().parse().expect("can not parse to int");

    min = std::cmp::min(input1, input2);
    max = std::cmp::max(input1, input2);

    for _ in 3..round + 1 {
        let input: i64 = read_input().parse().expect("can not parse to int");
        min = std::cmp::min(min, input);
        max = std::cmp::max(max, input);
    }

    println!("{}", min);
    println!("{}", max);
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let cleaned = input.trim().to_string();
    cleaned
}
