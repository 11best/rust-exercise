fn main() {
    let input_num = read_input();
    let mut splited_num: Vec<u8> = input_num
        .split(" ")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    splited_num.sort();

    let mut num_map: std::collections::HashMap<String, u8> = std::collections::HashMap::new();
    num_map.insert(String::from("A"), splited_num[0]);
    num_map.insert(String::from("B"), splited_num[1]);
    num_map.insert(String::from("C"), splited_num[2]);

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
