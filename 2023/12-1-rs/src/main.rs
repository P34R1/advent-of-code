fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("couldn't read ./input.txt");

    let answer_1 = december_1_rs::part_1(&input);
    let answer_2 = december_1_rs::part_2(&input);

    println!("1: {}", answer_1);
    println!("2: {}", answer_2);
}
