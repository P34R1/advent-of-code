use clipboard::{ClipboardContext, ClipboardProvider};

pub const LIB_RS_CONTENT: &[u8] = r#"use std::io::{Error, ErrorKind};

pub fn part_1(input: &str) -> Result<usize, Error> {
    Err(Error::new(ErrorKind::Other, "No Code"))
}

pub fn part_2(input: &str) -> Result<usize, Error> {
    Err(Error::new(ErrorKind::Other, "No Code"))
}
"#
.as_bytes();

pub fn input_txt_content(chosen_year: &str, chosen_day: &str) -> String {
    println!("Go to https://adventofcode.com/{chosen_year}/day/{chosen_day}/input, and copy the contents to your clipboard, then press any key");

    // Create a clipboard context
    let mut ctx = ClipboardContext::new().expect("Failed to create clipboard context");

    // Read a single key event
    loop {
        if let Ok(crossterm::event::Event::Key(crossterm::event::KeyEvent {
            code: crossterm::event::KeyCode::Char(_),
            ..
        })) = crossterm::event::read()
        {
            break;
        }
    }

    // Attempt to get the clipboard content as a String
    ctx.get_contents().expect("to get clipboard content")
}

pub fn cargo_toml_content(chosen_day: u32) -> String {
    format!(
        r#"[package]
name = "day-{chosen_day:02}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
"#
    )
}

pub fn main_rs_content(chosen_day: u32) -> String {
    format!(
        r#"fn main() {{
    let input = std::fs::read_to_string("./input.txt").expect("couldn't read ./input.txt");

    let answer_1 = day_{chosen_day:02}::part_1(&input);
    let answer_2 = day_{chosen_day:02}::part_2(&input);

    match answer_1 {{
        Ok(answer) => println!("1: {{}}", answer_1),
        Err(_) => {{}},
    }};

    match answer_2 {{
        Ok(answer) => println!("2: {{}}", answer_2),
        Err(_) => {{}},
    }};
}}
"#
    )
}
