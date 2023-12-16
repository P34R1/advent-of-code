const INPUT: &str = include_str!("../input.txt");

fn main() {
    let answer_1 = day_02::part_1(INPUT);
    let answer_2 = day_02::part_2(INPUT);

    match answer_1 {
        Ok(answer) => println!("1: {}", answer),
        Err(err) => eprintln!("1: {}", err),
    };

    match answer_2 {
        Ok(answer) => println!("2: {}", answer),
        Err(err) => eprintln!("2: {}", err),
    };
}
