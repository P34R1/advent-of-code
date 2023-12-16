fn convert_to_number(str: &str) -> usize {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => str.parse().expect("Valid Number"),
    }
}

pub fn part_1(input: &str) -> Result<usize, regex::Error> {
    let re = regex::Regex::new(r"(\d).*(\d)")?;
    let re2 = regex::Regex::new(r"(\d)")?;

    // Iterate through the lines in the input
    let calibration_values = input.lines().map(|str: &str| {
        let (first_num, last_num) = match re.captures(str) {
            Some(values) => (
                values.get(1).expect("Valid Match").as_str(),
                values.get(2).expect("Valid Match").as_str(),
            ),
            None => {
                let values = re2.captures(str).expect("regex");
                let first_num = values.get(1).expect("Valid Match").as_str();
                (first_num, first_num)
            }
        };

        // Combine the first and last numbers
        convert_to_number(first_num) * 10 + convert_to_number(last_num)
    });

    Ok(calibration_values.sum())
}

pub fn part_2(input: &str) -> Result<usize, regex::Error> {
    let re = regex::Regex::new(
        r"(one|two|three|four|five|six|seven|eight|nine|\d).*(one|two|three|four|five|six|seven|eight|nine|\d)",
    )?;
    let re2 = regex::Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)")?;

    // Iterate through the lines in the input
    let calibration_values = input.lines().map(|str: &str| {
        let values = match re.captures(str) {
            Some(v) => v,
            None => re2.captures(str).expect("regex"),
        };

        let first_num = values.get(1).expect("Valid Match").as_str();
        let last_num = if let Some(last_number) = values.get(2) {
            last_number.as_str()
        } else {
            first_num
        };

        // Combine the first and last numbers if they exist
        convert_to_number(first_num) * 10 + convert_to_number(last_num)
    });

    Ok(calibration_values.sum())
}
