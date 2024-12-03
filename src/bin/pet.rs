fn main() {
    let mut list: Vec<u8> = vec![0; 5];

    for x in 0..5 {
        let mut p = String::new();
        std::io::stdin().read_line(&mut p).expect("empty");
        let splited_num: Vec<u8> = p
            .trim()
            .split(" ")
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        let sum: u8 = splited_num.iter().sum();
        list[x] = sum
    }

    let max_score = list.iter().max().unwrap();
    let max_index = list
        .iter()
        .position(|element| element == max_score)
        .unwrap();

    println!("{} {}", max_index + 1, max_score);
}
