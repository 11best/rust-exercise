fn main() {
    let input_num = read_input();
    let mut splited_num: Vec<&str> = input_num.split_whitespace().collect();
    if splited_num.len() > 3 {
        panic!("invalid input num!")
    }

    splited_num.sort();
    let mut num_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    num_map.insert(String::from("A"), String::from(splited_num[0]));
    num_map.insert(String::from("B"), String::from(splited_num[1]));
    num_map.insert(String::from("C"), String::from(splited_num[2]));

    let input_abc = read_input();
    let splited_abc: Vec<String> = input_abc.chars().map(|c| c.to_string()).collect();

    for x in &splited_abc {
        print!("{} ", num_map.get(x).unwrap());
    }
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let cleaned = input.trim().to_string();
    cleaned
}
