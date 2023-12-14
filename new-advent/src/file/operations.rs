use std::{
    fs::{create_dir, File},
    io::{Error, ErrorKind, Write},
};

use chrono::{Datelike, NaiveDate};

use super::content::{cargo_toml_content, input_txt_content, main_rs_content, LIB_RS_CONTENT};

pub fn create_directories(chosen_advent: &NaiveDate) -> Result<(), Error> {
    let (chosen_year, chosen_day) = (chosen_advent.year(), chosen_advent.day());
    let dir_path = format!("{chosen_year}/day-{chosen_day:02}");

    match create_dir(chosen_year.to_string()) {
        Ok(_) => (),
        Err(err) => {
            if err.kind() != ErrorKind::AlreadyExists {
                panic!("{}", err)
            }
        }
    }

    create_dir(&dir_path)?;
    create_dir([&dir_path, "/src"].concat())?;

    Ok(())
}

pub fn create_files(chosen_advent: &NaiveDate) -> Result<(), Error> {
    let (chosen_year, chosen_day) = (chosen_advent.year(), chosen_advent.day());
    let dir_path = format!("{chosen_year}/day-{chosen_day:02}");

    let mut main_rs = File::create([&dir_path, "/src", "/main.rs"].concat())?;
    let mut lib_rs = File::create([&dir_path, "/src", "/lib.rs"].concat())?;
    let mut cargo_toml = File::create([&dir_path, "/Cargo.toml"].concat())?;
    let mut input_txt = File::create([&dir_path, "/input.txt"].concat())?;

    let main_rs_content = main_rs_content(chosen_day);
    let cargo_toml_content = cargo_toml_content(chosen_day);

    Write::write_all(&mut main_rs, main_rs_content.as_bytes())?;
    Write::write_all(&mut lib_rs, LIB_RS_CONTENT)?;
    Write::write_all(&mut cargo_toml, cargo_toml_content.as_bytes())?;

    let input_txt_content = input_txt_content(&chosen_year.to_string(), &chosen_day.to_string());
    Write::write_all(&mut input_txt, input_txt_content.as_bytes())?;

    Ok(())
}
