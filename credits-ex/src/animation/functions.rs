use rand::{RngExt, seq::IndexedRandom, rngs::ThreadRng};
use crate::animation::engine::{Generator, DataStore};
use crate::color::{Fore, Style, ansi_code};
use crate::canvas::{Canvas, Vector2};
use crate::weather::Weather;

pub fn generate_random_hex(length: usize) -> String {
    assert!((1..=16).contains(&length), "length out of range (expected 1..=16)");
    let mut rng: ThreadRng = rand::rng();
    
    format!("{:0length$x}", rng.random_range::<u128, _>(0..16_u128.pow(length as u32)))
}

pub fn replace_text_with_spaces(text: &str, chance: f64) -> String {
    let mut rng: ThreadRng = rand::rng();

    // Rust strings use UTF-8 byte offsets, while Python indexes Unicode code points.
    // Throughout this file, chars() keeps count(), nth() and take() based on Unicode
    // scalar values instead, so a multibyte symbol such as '°' advances one character
    // and is never split by a byte-based slice.
    text.chars()
        .map(|char: char| {
            if (rng.random_range::<i32, _>(0..=100) as f64) < chance { ' ' }
            else { char }
        })
        .collect::<String>()
}

pub fn render_weather(canvas: &mut Canvas, position: Vector2, weather: &mut Weather, steps: u32, chance: f64) {
    if weather.precipitation != -1.0 {
        canvas.set_string(
            position.offset(5, -2),
            &replace_text_with_spaces(&format!("{:>14}", weather.kind.as_str()), chance),
            &ansi_code(Fore::Yellow, Style::Normal)
        );
    }
    else {
        canvas.set_string(
            position.offset(0, -2),
            &replace_text_with_spaces(&format!("{:>14}", weather.kind.as_str()), chance),
            &ansi_code(Fore::Red, Style::Bright)
        );
    }

    let temp_colour: Fore = match (weather.temperature as i32).div_euclid(23).min(3) {
        0 | -4 => Fore::White,
        1 | -3 => Fore::Cyan,
        2 | -2 => Fore::Yellow,
        3 | -1 => Fore::Red,
        _      => panic!("temperature out of range"),
    };
    canvas.set_string(
        position.offset(0, 0),
        &replace_text_with_spaces(&format!("{:2}°F  ", weather.temperature as i32), chance),
        &ansi_code(temp_colour, Style::Bright),
    );

    canvas.set_string(
        position.offset(5, 0),
        &replace_text_with_spaces(&format!("Wind {:2} mph {}", weather.wind.wind_mph as i32, weather.wind.direction.as_str()), chance),
        &ansi_code(Fore::Cyan, Style::Bright),
    );

    canvas.set_string(
        position.offset(5, 2),
        &replace_text_with_spaces("Precipitation ", chance),
        &ansi_code(Fore::Blue, Style::Bright),
    );

    let mut percent: String = format!("{:.2}", weather.precipitation * 100.0);
    if percent.ends_with("0") { percent.pop(); }
    let text: String = format!("{:^13} ", format!("{percent}%"));
    canvas.set_string(
        position.offset(5, 3),
        &replace_text_with_spaces(&text, chance),
        &ansi_code(Fore::Blue, Style::Bright),
    );

    weather.mutate(steps);    
}

pub fn noise(canvas: &mut Canvas, amount: usize, chars: Vec<&str>, colors: Vec<&str>) {
    let mut rng: ThreadRng = rand::rng();
    
    for _ in 0..amount {
        let position: Vector2 = Vector2 {
            x: rng.random_range::<usize, _>(0..(canvas.dimensions.x)),
            y: rng.random_range::<usize, _>(0..(canvas.dimensions.y)),
        };

        canvas.set_char(
            position, 
            chars.choose(&mut rng).unwrap(), 
            colors.choose(&mut rng).unwrap(),
        );
    }
}

pub fn set_multiline_string(canvas: &mut Canvas, position: Vector2, string: &str, color: &str) {
    for (offset, line) in string.split("\n").enumerate() {
        canvas.set_string(
            position.offset(0, offset as isize), 
            line, 
            color,
        );
    }
}

pub fn type_text(canvas: &mut Canvas, generator: &mut Generator, position: Vector2, color: &str, render: bool) {
    let text_get: &str = generator.get_data::<String>("text")
        .expect("text not found");
    let offset: usize = *generator.get_data::<usize>("offset")
        .expect("offset not found");
    let mut total_chars: usize = 0;
    let mut add_offset: usize = 0;
    
    if text_get.is_empty() { return }

    match text_get.strip_prefix("[##CLEAR|") {
        Some(clear_bounds) => {
            let (clear_x, clear_y): (&str, &str) = clear_bounds.split('|').next().unwrap().split_once(';')
                .expect("invalid clear bounds");

            let clear_x: usize = clear_x.parse::<usize>()
                .expect("invalid clear x");
            let clear_y: usize = clear_y.parse::<usize>()
                .expect("invalid clear y");

            if render {
                let spaces: String = " ".repeat(clear_x);

                for yclear in 0..clear_y {
                    canvas.set_string(
                        position.offset(0, yclear as isize),
                        &spaces,
                        color,
                    );
                }
            }
        },
        None => {
            for (linecount, text_line) in text_get.split("\n").enumerate() {
                if let Some(local_offset) = offset.checked_sub(total_chars)
                    .filter(|&n: &usize| n < text_get.chars().count()) 
                {
                    let typer: &str = match local_offset < text_line.chars().count().saturating_sub(1) {
                        true => "_",
                        false => "",
                    };
                    let mut string: String = text_line.chars().take(local_offset).collect::<String>() + typer;

                    if text_line.chars().nth(local_offset) == Some('@') {
                        add_offset += 3;
                    }

                    string = string.replace("~", "").replace("@", "");
                    total_chars += text_line.chars().count();

                    if render {
                        canvas.set_string(
                            position.offset(0, linecount as isize), 
                            &string, 
                            color
                        );
                    }
                }
            }

            *generator.get_data_mut::<usize>("offset")
                .expect("offset not found") 
            += 1 + add_offset;
        },
    }
}

// ...Wait, what?
pub fn fuck_up_text(string: &str, chance: f64, also_ignore: &str) -> String {
    let mut rng: ThreadRng = rand::rng();

    let total_ignore: String = "@~\n".to_string() + also_ignore;
    let mut new_str: String = String::new();
    let fucks: Vec<char> = vec![
        '.', '.', '.', ' ', ' ', '`', '=', '/', '?', '-', '$', '%'
    ];

    for char in string.chars() {
        match char {
            // total_ignore already contains '\n'.
            // Maybe we could remove this arm
            // (but it won't advance the RNG state if needed)
            // or remove '\n' from total_ignore? 
            // To preserve the original behavior I kept it.
            '\n' => new_str.push(char),
            char if (rng.random_range::<i32, _>(1..=1000) as f64) < chance
                && !total_ignore.contains(char) 
                    => new_str.push(*fucks.choose(&mut rng).unwrap()),
            _ => new_str.push(char),
        }
    }

    new_str
}

pub fn clear(canvas: &mut Canvas) {
    canvas.clear_layer();
}

pub fn beat_toggle(canvas: &mut Canvas, generator: &mut Generator, position_1: Vector2, position_2: Vector2, char: &str, color: &str) {
    let toggle: bool = *generator.get_data::<bool>("beat_toggle")
        .expect("toggle not found");
    let chars: &str = if toggle { char } else { ".." };
    let dx: usize = position_2.x - position_1.x;
    let line: String = chars.repeat(dx);

    for yn in position_1.y..position_2.y {
        let position: Vector2 = Vector2 {
            x: position_1.x,
            y: yn,
        };
        canvas.set_string(position, &line, color);
    }

    generator.set_data::<bool>("beat_toggle", !toggle);
}

pub fn work_out_date(beats: i32, day_offset: usize) -> String {
    let lengths: [u32; 12] = [
        31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31
    ];

    let mut days_needed: u32 = ((beats / 64) + day_offset as i32) as u32;

    let mut day_local: u32 = 22;
    let mut month_local: u32 = 9;
    let mut year_local: u32 = 2009;

    while days_needed > 0 {
        let leap_year: u32 = if year_local % 4 == 0 && month_local == 1 { 1 } else { 0 };
        let until_end_month: u32 = lengths[month_local as usize] + 1 - day_local + leap_year;
        let days_passed: u32 = days_needed.min(until_end_month);

        day_local += days_passed;
        days_needed -= days_passed;

        if day_local > lengths[month_local as usize] + leap_year {
            month_local += 1;
            day_local = 1;
            if month_local >= 12 {
                month_local = 0;
                year_local += 1;
            }
        }
    }

    format!("{:02}.{:02}.{:04}",day_local, month_local + 1, year_local)
}

pub fn split_word_template(string: &str) -> Vec<Vec<String>> {
    string.split("\n")
        .map(|sp: &str| sp.split("#").map(str::to_string).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

pub fn typewrite_by_word(canvas: &mut Canvas, manager_data: &mut DataStore, generator: &mut Generator, position: Vector2, color: &str, render: bool, history_variable: &str) {
    let text_get: &Vec<Vec<String>> = generator.get_data::<Vec<Vec<String>>>("text")
        .expect("text not found");
    let offset: usize = *generator.get_data::<usize>("offset")
        .expect("offset not found");
    let line_no: usize = *generator.get_data::<usize>("line_no")
        .expect("lineno not found");
    // Intentional extension of the original Python: clear_line defaults to true.
    // Setting it to false skips automatic line clearing, so new text overwrites its prefix only and leaves the remaining original text visible.
    // Explicit [~~CLEAR|...] commands still clear their requested area.
    let clear_line: bool = generator.get_data::<bool>("clear_line")
        .copied().unwrap_or(true);
    
    if !text_get.is_empty() && line_no < text_get.len() {
        let line_get: &Vec<String> = &text_get[line_no];
        let line_total: String = line_get[..offset.min(line_get.len())].concat();
        
        if line_get.is_empty() {
            generator.set_data::<usize>("offset", offset + 1);
        }
        else {
            match line_get[0].strip_prefix("[~~CLEAR|") {
                Some(_) => {
                    let clear_bounds: &str = line_get[0].split("|").nth(1).unwrap();
                    let clear_bounds: usize = clear_bounds.parse::<usize>()
                        .expect("invalid clear_bounds");

                    let string: &str = &" ".repeat(clear_bounds);

                    if render {
                        canvas.set_string(
                            position,
                            string,
                            color
                        );
                    }
                }
                None => {
                    if render && (clear_line || !line_total.is_empty()) {
                        match line_total.is_empty() {
                            true => canvas.set_string(
                                position,
                                &" ".repeat(60),
                                color),
                            false => canvas.set_string(
                                position,
                                &line_total.replace("~", ""),
                                color
                            ),
                        }
                    }

                    if offset >= line_get.len() {
                        let is_fluff: bool = line_total.starts_with(" ") || line_total.is_empty();
                        let is_important: bool = line_total.ends_with("~");
                        let (prefix, color_select): (&str, String) = match (is_fluff, is_important) {
                            (true, _) => ("  ", ansi_code(Fore::Black, Style::Bright)),
                            (_, true) => ("> ", ansi_code(Fore::Green, Style::Normal)),
                            _ => ("- ", ansi_code(Fore::Yellow, Style::Normal)),
                        }; 

                        let entry: (String, String) = (format!("{}{}", prefix, line_total.trim_matches(' ').replace("~", "")), color_select);

                        if let Some(history) = manager_data.get_mut::<Vec<(String, String)>>(history_variable) {
                            history.push(entry);
                        } 
                        else {
                            manager_data.set::<Vec<(String, String)>>(history_variable, vec![entry]);
                        }

                        manager_data.set::<bool>("refresh", true);

                        generator.set_data::<usize>("offset", 0);
                        generator.set_data::<usize>("line_no", line_no + 1);
                    }
                    else {
                        generator.set_data::<usize>("offset", offset + 1);
                    }
                }
            }
        }
    }
}

pub fn write_history(canvas: &mut Canvas, manager_data: &mut DataStore, position: Vector2, stop: usize, history_variable: &str) {
    let need_refresh: bool = manager_data.get::<bool>("refresh").copied().unwrap_or(false);

    if need_refresh {
        manager_data.set::<bool>("refresh", false);

        match manager_data.get::<Vec<(String, String)>>(history_variable) {
            Some(history) if !history.is_empty() => {
                let mut line_id: usize = 0;
                for y in (stop..=position.y).rev() {
                    let line: &(String, String) = match line_id < history.len() {
                        true => &history[history.len() - 1 - line_id],
                        false => &(String::new(), ansi_code(Fore::Black, Style::Bright)),
                    };
                    line_id += 1;

                    canvas.set_string(
                        Vector2 { x: position.x, y},
                        &format!("{:50}", line.0),
                        &line.1,
                    );
                }
            } 
            _ => {
                for y in (stop..=position.y).rev() {
                    canvas.set_string(Vector2 { x: position.x, y },
                        &" ".repeat(50),
                        &ansi_code(Fore::Black, Style::Normal),
                    );
                }
            }
        }
    }
}

pub fn show_access_point_visual(canvas: &mut Canvas, generator: &mut Generator, position: Vector2) {
    let counter: usize = *generator.get_data::<usize>("counter")
        .expect("counter not found");
    let block_counter: usize = *generator.get_data_mut::<usize>("block")
        .expect("block not found");

    if counter < 8 {
        set_multiline_string(
            canvas,
            position.offset(
                5 * (block_counter % 6) as isize,
                4 * (block_counter / 6) as isize,
            ),
            &format!("  ###  \nPBS #{:02}\nPing  {}", block_counter + 1, counter + 1),
            &ansi_code(Fore::Yellow, Style::Normal)
        );

        *generator.get_data_mut::<usize>("counter")
            .expect("counter not found")
        += 1;
    }
    else {
        set_multiline_string(
            canvas,
            position.offset(
                5 * (block_counter % 6) as isize,
                4 * (block_counter / 6) as isize),
                &format!("  ...  \nPBS #{:02}\n-------", block_counter + 1),
                &ansi_code(Fore::Black, Style::Bright)
        );

        set_multiline_string(
            canvas,
            position.offset(
                5 * ((block_counter + 1) % 6) as isize,
                4 * ((block_counter + 1) / 6) as isize,
            ),
            &format!("  ###  \nPBS #{:02}\nPing  {}", block_counter + 2, 1),
            &ansi_code(Fore::Yellow, Style::Normal)
        );

        generator.set_data::<usize>("counter", 1);
        *generator.get_data_mut::<usize>("block")
            .expect("block not found") 
        += 1;
    }
}

pub fn make_poweroff_bars(canvas: &mut Canvas, beats: i32, color: &str) {
    let height: usize = (24.0 / (beats as f64).powf(1.3)) as usize;
    let position: Vector2 = Vector2 {
        x: 0,
        y: 12 - (height / 2),
    };

    canvas.set_string(
        position,
        &"##".repeat(40 * height),
        color
    );
}
