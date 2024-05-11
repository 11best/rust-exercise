fn main() {
    let round: u16 = read_input().parse().expect("can not parse to int");
    if (round > 1000) {
        panic!("n is more than thousand!")
    }
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let cleaned = input.trim().to_string();
    cleaned
}

fn read_input_u_int() -> u8 {
    read_input().parse().expect("can not parse to u int")
}

fn is_valid_score(score: u8, max_score: u8) -> bool {
    score <= max_score
}

fn read_input_score(max_score: u8) -> u8 {
    let input_score = read_input_u_int();
    if !is_valid_score(input_score, max_score) {
        panic!("score is invalid!")
    }
    input_score
}
