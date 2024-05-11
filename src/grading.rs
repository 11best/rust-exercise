
fn main() {
    let collected_score:u8 = read_input_score(30);
    let midterm_score:u8 = read_input_score(30);
    let final_score:u8 = read_input_score(40);

    grading_score(collected_score, midterm_score, final_score)
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

fn is_valid_score(score: u8, max_score: u8) -> bool{
    score <= max_score
}

fn read_input_score(max_score: u8) -> u8 {
    let input_score = read_input_u_int();
    if !is_valid_score(input_score, max_score) {
        panic!("score is invalid!")
    }
    input_score
}

fn grading_score(collected_score: u8, midterm_score: u8, final_score: u8) {
    let sum_score = collected_score + midterm_score + final_score;
    match sum_score {
        s if s >= 80 && s <= 100 => println!("A"),
        s if s >= 75 && s <= 79 => println!("B+"),
        s if s >= 70 && s <= 74 => println!("B"),
        s if s >= 65 && s <= 69 => println!("C+"),
        s if s >= 60 && s <= 64 => println!("C"),
        s if s >= 55 && s <= 59 => println!("D+"),
        s if s >= 50 && s <= 54 => println!("D"),
        s if s <= 49 => println!("F"),
        _ => panic!("sum score is invalid!"),
    }
}