use crate::{DAY_SHORT, INI_DATE_FORMAT, INI_FILE_PATH, TIME_SHORT};
use chrono::NaiveDate;
use ini::Ini;
use log::info;
use std::array;
use std::path::Path;

pub type WeekData = [[String; 2]; 7];

// # Rules for WeekDate persistence
//
// - Absent menu.ini is treated the same as a empty menu.ini.
// - Absent week-section is treated the same as a empty week-section.
// - A week-section with one entry with a single non-empty entry will be written with all entries.
// - Entries will be trimmed on load, save and before comparing.

pub fn create_empty_week() -> WeekData {
    // Initialize 2D array, from: https://users.rust-lang.org/t/how-to-init-2d-array-using-function/80737/2
    let week_string: WeekData = array::from_fn(|_y| array::from_fn(|_x| "".to_string()));
    week_string
}

pub fn load_week(datum: &NaiveDate) -> WeekData {
    let date_string = datum.format(INI_DATE_FORMAT).to_string();
    let ini_path = Path::new(INI_FILE_PATH);
    let conf: Ini = match ini_path.exists() {
        true => Ini::load_from_file(ini_path).expect("Error loading ini file"),
        false => Ini::new(),
    };

    let mut week_string = create_empty_week();
    for (y, day) in DAY_SHORT.iter().enumerate() {
        for (x, time) in TIME_SHORT.iter().enumerate() {
            let key = format!("{day}_{time}");
            week_string[y][x] = conf
                .get_from_or(Some(date_string.as_str()), key.as_str(), "")
                .to_owned();
        }
    }
    info!("Loading {}", date_string);
    week_string
}

fn save_week(week_data: &WeekData, date_str: &str) {
    let ini_path = Path::new(INI_FILE_PATH);
    let mut conf: Ini = match ini_path.exists() {
        true => Ini::load_from_file(ini_path).expect("Error loading ini file"),
        false => Ini::new(),
    };

    let mut section = conf.with_section(Some(date_str));
    for (y, day) in DAY_SHORT.iter().enumerate() {
        for (x, time) in TIME_SHORT.iter().enumerate() {
            let key = format!("{day}_{time}");
            section.set(key, week_data[y][x].trim());
        }
    }
    info!("Saving {}", date_str);
    conf.write_to_file(INI_FILE_PATH)
        .expect("Error writing ini file");
}

pub fn save_if_needed(week_data: &WeekData, datum: &NaiveDate) {
    let date_string = datum.format(INI_DATE_FORMAT).to_string();
    if !is_equal_to_saved(week_data, datum) {
        info!("Data changed, saving data");
        save_week(week_data, date_string.as_str());
    }
}

pub fn is_equal_to_saved(week_data: &WeekData, datum: &NaiveDate) -> bool {
    let week_datum = load_week(datum);
    is_week_equal(week_data, &week_datum)
}

pub fn is_week_equal(week_data1: &WeekData, week_data2: &WeekData) -> bool {
    for y in 0..DAY_SHORT.len() {
        for x in 0..TIME_SHORT.len() {
            if week_data1[y][x].trim() != week_data2[y][x].trim() {
                info!(
                    "week_data1[{}][{}] != week_data2[{}][{}], '{}' != '{}'",
                    y, x, y, x, week_data1[y][x], week_data2[y][x]
                );
                return false;
            }
        }
    }
    true
}
