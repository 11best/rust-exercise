fn main() {
    let input_string = read_input();
    let splited_num: Vec<&str> = input_string.split_whitespace().collect();
    let x1: i16 = splited_num[0].parse().expect("can not parse to int");
    let s: i16 = splited_num[1].parse().expect("can not parse to int");

    let t = find_x2(x1, s).expect("can not find x2");
    let x = Some(t);
    assert_eq!(x.ok_or("No value found"), Ok(t));
    println!("{}", t)
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let cleaned = input.trim().to_string();
    cleaned
}

fn find_x2(x1: i16, s: i16) -> Result<i16, std::fmt::Error> {
    let x2: i16 = (s * 2) - x1;
    Ok(x2)
}
