fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("couldn't read ./input.txt");

    let answer_1 = december_2_rs::part_1(&input);
    let answer_2 = december_2_rs::part_2(&input);

    println!("1: {}\n2: {}", answer_1, answer_2);
}