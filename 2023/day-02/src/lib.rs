const MAX_CUBES_RED: usize = 12;
const MAX_CUBES_GREEN: usize = 13;
const MAX_CUBES_BLUE: usize = 14;

#[derive(Default)]
pub struct Set {
    red: usize,
    green: usize,
    blue: usize,
}

fn process_game(game: &str) -> impl Iterator<Item = Set> + '_ {
    // Extract the set information for each game
    game.split(": ")
        .last()
        .expect("get last item")
        .split("; ")
        .map(|set| {
            // Process each set to extract number and color information
            let mut set_return = Set::default();

            for item in set.split(", ") {
                let mut item = item.split(' ');
                let number = item
                    .next()
                    .expect("get number")
                    .parse::<usize>()
                    .expect("get number as usize");
                let colour = item.last().expect("get colour");

                // Match the color and assign the number accordingly
                match colour {
                    "red" => set_return.red = number,
                    "green" => set_return.green = number,
                    "blue" => set_return.blue = number,
                    _ => panic!("Invalid String"),
                }
            }

            set_return // Return the processed set
        })
}

pub fn part_1(input: &str) -> Result<usize, std::io::Error> {
    let games = input.lines().map(process_game);

    let valid_games = games.enumerate().filter_map(|(i, mut game)| {
        if game.all(|set| {
            set.red <= MAX_CUBES_RED && set.green <= MAX_CUBES_GREEN && set.blue <= MAX_CUBES_BLUE
        }) {
            Some(i + 1)
        } else {
            None
        }
    });

    Ok(valid_games.sum())
}

pub fn part_2(input: &str) -> Result<usize, std::io::Error> {
    let games = input.lines().map(process_game);

    let power_of_games = games.map(|game| {
        let (mut max_red, mut max_green, mut max_blue) = (usize::MIN, usize::MIN, usize::MIN);

        // Iterate through the data to find maximum values for each element
        game.for_each(|Set { red, green, blue }| {
            max_red = max_red.max(red);
            max_green = max_green.max(green);
            max_blue = max_blue.max(blue);
        });

        max_red * max_green * max_blue
    });

    Ok(power_of_games.sum())
}
