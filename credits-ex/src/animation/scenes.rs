use rand::{RngExt, rngs::ThreadRng};

use crate::canvas::Vector2;
use crate::animation::engine::{Condition, Context, Generator, Scene};
use crate::animation::functions::*;
use crate::animation::ocean::{begin_ocean, update_ocean_slices};
use crate::color::{Fore, Style, ansi_code};
use crate::weather::Weather;

pub fn wipe() -> Scene {
    Scene::init("wipe", vec![
        Generator::init(
            0,
            Condition::Always,
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                let bright_white: String = ansi_code(Fore::White, Style::Bright);
                let normal_white: String = ansi_code(Fore::White, Style::Normal);
                let bright_black: String = ansi_code(Fore::Black, Style::Bright);
                let mut colors: Vec<&str> = Vec::new();
                colors.extend(vec![bright_white.as_str(); beat.max(0) as usize]);
                colors.extend(vec![normal_white.as_str(); (70 - beat).max(0) as usize]);
                colors.extend(vec![bright_black.as_str(); 4 * (40 - beat).max(0) as usize]);

                noise(
                    context.canvas,
                    (beat as f64).powf(1.4) as usize,
                    vec!["##", "@@", "  "],
                    colors
                );
            },
            None,
        ),
    ])
}

pub fn clear_wipe() -> Scene {
    Scene::init("clear_wipe", vec![
        Generator::init(
            0,
            Condition::Always,
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                noise(
                    context.canvas,
                    (beat as f64).powf(2.2) as usize,
                    vec!["  "],
                    vec![&ansi_code(Fore::White, Style::Bright)]
                );
            },
            None,
        ),
    ])
}

pub fn clear_scene() -> Scene {
    Scene::init("clear", vec![
        Generator::init(
            0,
            Condition::Always,
            Some(|_, context: &mut Context<'_>| {
                clear(
                    context.canvas
                );
            }),
            |_, _, _| {},
            None,
        ),
    ])
}

pub fn ocean() -> Scene {
    Scene::init("ocean_b", vec![
        Generator::init(
            0,
            Condition::Every(2),
            Some(|generator: &mut Generator, context: &mut Context<'_>| {
                let ocean_time: &mut usize = context.manager_data.get_mut::<usize>("ocean_time")
                    .expect("ocean_time not found");
                let ocean: Vec<String> = begin_ocean(ocean_time);
                generator.set_data::<Vec<String>>("ocean", ocean);
                generator.set_data::<f64>("ocean_glitch", 1.0);
                generator.set_data::<String>("ocean_col", ansi_code(Fore::Blue, Style::Bright));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let ocean_glitch: f64 = *generator.get_data::<f64>("ocean_glitch")
                    .expect("ocean_glitch not found");
                let ocean_col: String = generator.get_data::<String>("ocean_col")
                    .expect("ocean_col not found").clone();
                let ocean_content: &mut Vec<String> = generator.get_data_mut::<Vec<String>>("ocean")
                    .expect("ocean not found");
                let ocean_time: &mut usize = context.manager_data.get_mut::<usize>("ocean_time")
                    .expect("ocean_time not found");
                // Intentional change from the original Python: keep incoming columns at the
                // base letter probability, then let mutate_text apply the current transition
                // strength to them along with the rest of the ocean.
                let string: String = update_ocean_slices(ocean_content, ocean_glitch, 1.0, ocean_time);

                context.canvas.set_string(
                    Vector2::init(0, 14),
                    &string,
                    &ocean_col
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<String>("text", String::new());
                generator.set_data::<usize>("offset", 0);
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                type_text(
                    context.canvas,
                    generator,
                    Vector2::init(1, 1),
                    &ansi_code(Fore::White, Style::Bright),
                    true
                );
            },
            None,
        ),
    ])
}

pub fn ocean2() -> Scene {
    Scene::init("ocean_c", vec![
        Generator::init(
            0,
            Condition::Every(2),
            Some(|generator: &mut Generator, context: &mut Context<'_>| {
                let ocean_time: &mut usize = context.manager_data.get_mut::<usize>("ocean_time")
                    .expect("ocean_time not found");
                let ocean: Vec<String> = begin_ocean(ocean_time);
                generator.set_data::<Vec<String>>("ocean", ocean);
                generator.set_data::<f64>("ocean_glitch", 16.0);
                generator.set_data::<String>("ocean_col", ansi_code(Fore::Black, Style::Bright));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let ocean_glitch: f64 = *generator.get_data::<f64>("ocean_glitch")
                    .expect("ocean_glitch not found");
                let ocean_col: String = generator.get_data::<String>("ocean_col")
                    .expect("ocean_col not found").clone();
                let ocean_content: &mut Vec<String> = generator.get_data_mut::<Vec<String>>("ocean")
                    .expect("ocean not found");
                let ocean_time: &mut usize = context.manager_data.get_mut::<usize>("ocean_time")
                    .expect("ocean_time not found");
                let string: String = update_ocean_slices(ocean_content, ocean_glitch, ocean_glitch, ocean_time);

                context.canvas.set_string(
                    Vector2::init(0, 14),
                    &string,
                    &ocean_col
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<String>("text", String::new());
                generator.set_data::<usize>("offset", 0);
            }),
            |generator: &mut Generator, beat: i32, context: &mut Context<'_>| {
                type_text(
                    context.canvas,
                    generator,
                    Vector2::init(1, 1),
                    &ansi_code(Fore::Red, Style::Bright),
                    ![0, 8, 17, 15].contains(&((beat as f64).powf(1.143) as i32).rem_euclid(20))
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<String>("text", String::new());
                generator.set_data::<usize>("offset", 0);
            }),
            |generator: &mut Generator, beat: i32, context: &mut Context<'_>| {
                type_text(
                    context.canvas,
                    generator,
                    Vector2::init(1, 1),
                    &ansi_code(Fore::Red, Style::Bright),
                    [0, 15].contains(&((beat as f64).powf(1.2) as i32).rem_euclid(20))
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<String>("text", String::new());
                generator.set_data::<usize>("offset", 0);
            }),
            |generator: &mut Generator, beat: i32, context: &mut Context<'_>| {
                type_text(
                    context.canvas,
                    generator,
                    Vector2::init(1, 1),
                    &ansi_code(Fore::Red, Style::Bright),
                    ((beat as f64).powf(1.2) as i32).rem_euclid(20) == 8
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<String>("text", String::new());
                generator.set_data::<usize>("offset", 0);
            }),
            |generator: &mut Generator, beat: i32, context: &mut Context<'_>| {
                type_text(
                    context.canvas,
                    generator,
                    Vector2::init(1, 1),
                    &ansi_code(Fore::Red, Style::Bright),
                    ((beat as f64).powf(1.2) as i32).rem_euclid(20) == 17
                );
            },
            None,
        ),
    ])
}

pub fn ocean3() -> Scene {
    Scene::init("ocean_d", vec![
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, context: &mut Context<'_>| {
                let ocean_time: &mut usize = context.manager_data.get_mut::<usize>("ocean_time")
                    .expect("ocean_time not found");
                let ocean: Vec<String> = begin_ocean(ocean_time);
                generator.set_data::<Vec<String>>("ocean", ocean);
                generator.set_data::<f64>("ocean_glitch", 100.0);
                generator.set_data::<String>("ocean_col", ansi_code(Fore::Magenta, Style::Normal));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let ocean_glitch: f64 = *generator.get_data::<f64>("ocean_glitch")
                    .expect("ocean_glitch not found");
                let ocean_col: String = generator.get_data::<String>("ocean_col")
                    .expect("ocean_col not found").clone();
                let ocean_content: &mut Vec<String> = generator.get_data_mut::<Vec<String>>("ocean")
                    .expect("ocean not found");
                let ocean_time: &mut usize = context.manager_data.get_mut::<usize>("ocean_time")
                    .expect("ocean_time not found");
                let string: String = update_ocean_slices(ocean_content, ocean_glitch, ocean_glitch, ocean_time);

                context.canvas.set_string(
                    Vector2::init(0, 14),
                    &string,
                    &ocean_col
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            None,
            |_, _, context: &mut Context<'_>| {
                let blocks: String = (0..4)
                    .map(|_| (0..8).map(|_| generate_random_hex(4)).collect::<Vec<String>>().join(" "))
                    .collect::<Vec<String>>().join("\n");
                let string: String = "The system has encountered a fatal error. Please wait.\n\n[ERR: 801]\n\n".to_string() + &blocks;

                set_multiline_string(
                    context.canvas,
                    Vector2::init(1, 1),
                    &string,
                    &ansi_code(Fore::Red, Style::Bright)
                );
            },
            None,
        ),
    ])
}

pub fn text() -> Scene {
    Scene::init("typewrite", vec![
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<String>("text", String::new());
                generator.set_data::<usize>("offset", 0);
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                type_text(
                    context.canvas,
                    generator,
                    Vector2::init(1, 1),
                    &ansi_code(Fore::White, Style::Bright),
                    true
                );
            },
            None,
        ),
    ])
}

pub fn title() -> Scene {
    Scene::init("title", vec![
        Generator::init(
            64,
            Condition::At(64),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(1, 1),
                    "animation | plaaosert",
                    &ansi_code(Fore::Cyan, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            128,
            Condition::At(128),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(1, 2),
                    "bgm       | Frums - Credits",
                    &ansi_code(Fore::Cyan, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            192,
            Condition::At(192),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(1, 3),
                    "remake    | KnowHT",
                    &ansi_code(Fore::Cyan, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            256,
            Condition::At(256),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(1, 5),
                    "running pure Rust 1.96",
                    &ansi_code(Fore::Cyan, Style::Normal)
                );
            },
            None,
        ),
        Generator::init(
            320,
            Condition::At(320),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(1, 6),
                    "in the command line",
                    &ansi_code(Fore::Cyan, Style::Normal)
                );
            },
            None,
        ),
        Generator::init(
            384,
            Condition::At(384),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(0, 21),
                    "--------------------------------------------------------------------------------",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            448,
            Condition::At(448),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(1, 22),
                    "> _ ",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            512,
            Condition::At(512),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(26, 14),
                    "----------------------------\n|                          |\n----------------------------\n|                          |\n|                          |\n|                          |\n|                          |\n",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            576,
            Condition::At(576),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(27, 15),
                    "22.10.2009",
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            640,
            Condition::At(640),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(27, 17),
                    "43°F      Wind 13 mph W \n                        ",
                    &ansi_code(Fore::Cyan, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            704,
            Condition::At(704),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(32, 19),
                    "Precipitation \n    20.3%     ",
                    &ansi_code(Fore::Blue, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            768,
            Condition::At(768),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(36, 15),
                    "Cloudy",
                    &ansi_code(Fore::Yellow, Style::Normal)
                );
            },
            None,
        ),
    ])
}

pub fn beats() -> Scene {
    Scene::init("beats", vec![
        Generator::init(
            0,
            Condition::All(vec![Condition::EveryOnOff(48, 16), Condition::Every(4)]),
            Some(|generator: &mut Generator, _| {
                generator.set_data::<bool>("beat_toggle", true);
            }),
            |generator: &mut Generator, _beat: i32, context: &mut Context<'_>| {
                beat_toggle(
                    context.canvas,
                    generator,
                    Vector2::init(18, 11),
                    Vector2::init(22, 15),
                    "@@",
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::All(vec![Condition::EveryOffOn(48, 16), Condition::Every(2)]),
            Some(|generator: &mut Generator, _| {
                generator.set_data::<bool>("beat_toggle", true);
            }),
            |generator: &mut Generator, _beat: i32, context: &mut Context<'_>| {
                beat_toggle(
                    context.canvas,
                    generator,
                    Vector2::init(18, 11),
                    Vector2::init(22, 15),
                    "##",
                    &ansi_code(Fore::Green, Style::Bright)
                );
            },
            None,
        ),
    ])
}

pub fn beats_lr() -> Scene {
    Scene::init("beats_lr", vec![
        Generator::init(
            0,
            Condition::All(vec![Condition::EveryOnOff(48, 16), Condition::Every(4)]),
            Some(|generator: &mut Generator, _| {
                generator.set_data::<bool>("beat_toggle", true);
            }),
            |generator: &mut Generator, _beat: i32, context: &mut Context<'_>| {
                beat_toggle(
                    context.canvas,
                    generator,
                    Vector2::init(18, 11),
                    Vector2::init(20, 15),
                    "@@",
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::All(vec![Condition::EveryOffOn(48, 16), Condition::Every(2)]),
            Some(|generator: &mut Generator, _| {
                generator.set_data::<bool>("beat_toggle", true);
            }),
            |generator: &mut Generator, _beat: i32, context: &mut Context<'_>| {
                beat_toggle(
                    context.canvas,
                    generator,
                    Vector2::init(20, 11),
                    Vector2::init(22, 15),
                    "##",
                    &ansi_code(Fore::Green, Style::Bright)
                );
            },
            None,
        ),
    ])
}

pub fn beats_side() -> Scene {
    Scene::init("beats_side", vec![
        Generator::init(
            0,
            Condition::All(vec![Condition::EveryOnOff(48, 16), Condition::Every(4)]),
            Some(|generator: &mut Generator, _| {
                generator.set_data::<bool>("beat_toggle", true);
            }),
            |generator: &mut Generator, _beat: i32, context: &mut Context<'_>| {
                beat_toggle(
                    context.canvas,
                    generator,
                    Vector2::init(18, 11),
                    Vector2::init(20, 15),
                    "@@",
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::All(vec![Condition::EveryOffOn(48, 16), Condition::Every(2)]),
            Some(|generator: &mut Generator, _| {
                generator.set_data::<bool>("beat_toggle", true);
            }),
            |generator: &mut Generator, _beat: i32, context: &mut Context<'_>| {
                beat_toggle(
                    context.canvas,
                    generator,
                    Vector2::init(20, 11),
                    Vector2::init(22, 15),
                    "##",
                    &ansi_code(Fore::Green, Style::Bright)
                );
            },
            None,
        ),
    ])
}

pub fn date_ticker() -> Scene {
    Scene::init("dates", vec![
        Generator::init(
            0,
            Condition::Before(512),
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(27, 15),
                    &work_out_date(beat, 0),
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            512,
            Condition::Before(764),
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(27, 15),
                    &work_out_date(64 * 8 + (beat - 64 * 8) * 2, 0),
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            764,
            Condition::At(764),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(27, 15),
                    "??.??.????",
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            772,
            Condition::All(vec![Condition::Every(4), Condition::Before(1024)]),
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                let mut rng: ThreadRng = rand::rng();
                context.canvas.set_string(
                    Vector2::init(27, 15),
                    &work_out_date(rng.random_range::<i32, _>(64 * 12 + beat * 8..=64 * 12 + beat * 24), 0),
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            1024,
            Condition::Always,
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(27, 15),
                    &work_out_date(64 * 16 + beat * 64 - 64 * 16 * 32, 0),
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
    ])
}

pub fn redraw_ui() -> Scene {
    Scene::init("redraw_ui", vec![
        Generator::init(
            0,
            Condition::At(0),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(0, 21),
                    "--------------------------------------------------------------------------------",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::At(0),
            None,
            |_, _, context: &mut Context<'_>| {
                context.canvas.set_string(
                    Vector2::init(1, 22),
                    "> _ ",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::At(2),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(26, 14),
                    "----------------------------\n|                          |\n|                          |\n|                          |\n|                          |\n|                          |\n|                          |\n",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::At(3),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(27, 15),
                    "22.10.2009",
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::At(4),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(26, 16),
                    "----------------------------",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::At(5),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(27, 17),
                    "43°F      Wind 13 mph W ",
                    &ansi_code(Fore::Cyan, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::At(6),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(32, 19),
                    "Precipitation \n    20.3%     ",
                    &ansi_code(Fore::Blue, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::At(7),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(36, 15),
                    "Cloudy",
                    &ansi_code(Fore::Yellow, Style::Normal)
                );
            },
            None,
        ),
    ])
}

pub fn weather() -> Scene {
    let mut weather: Scene = Scene::init("weather", vec![
        Generator::init(
            0,
            Condition::All(vec![Condition::Every(64), Condition::Before(512)]),
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                let known_weathers: &mut [Weather; 5] = context.scene_data.get_mut::<[Weather; 5]>("known_weathers")
                    .expect("known_weathers not found");
                render_weather(
                    context.canvas,
                    Vector2::init(27, 17),
                    &mut known_weathers[(beat / 64).min(2) as usize],
                    if beat < 128 { 0 } else { 1 },
                    0.0
                );
            },
            None,
        ),
        Generator::init(
            512,
            Condition::All(vec![Condition::Every(32), Condition::Before(768)]),
            None,
            |_, _, context: &mut Context<'_>| {
                let known_weathers: &mut [Weather; 5] = context.scene_data.get_mut::<[Weather; 5]>("known_weathers")
                    .expect("known_weathers not found");
                render_weather(
                    context.canvas,
                    Vector2::init(27, 17),
                    &mut known_weathers[2],
                    1,
                    0.0
                );
            },
            None,
        ),
        Generator::init(
            764,
            Condition::At(764),
            None,
            |_, _, context: &mut Context<'_>| {
                let known_weathers: &mut [Weather; 5] = context.scene_data.get_mut::<[Weather; 5]>("known_weathers")
                    .expect("known_weathers not found");
                render_weather(
                    context.canvas,
                    Vector2::init(27, 17),
                    &mut known_weathers[4],
                    1,
                    0.0
                );
            },
            None,
        ),
        Generator::init(
            772,
            Condition::All(vec![Condition::Every(4), Condition::Before(1024)]),
            None,
            |_, _, context: &mut Context<'_>| {
                let known_weathers: &mut [Weather; 5] = context.scene_data.get_mut::<[Weather; 5]>("known_weathers")
                    .expect("known_weathers not found");
                render_weather(
                    context.canvas,
                    Vector2::init(27, 17),
                    &mut known_weathers[2],
                    14,
                    0.0
                );
            },
            None,
        ),
        Generator::init(
            1024,
            Condition::Always,
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                let known_weathers: &mut [Weather; 5] = context.scene_data.get_mut::<[Weather; 5]>("known_weathers")
                    .expect("known_weathers not found");
                render_weather(
                    context.canvas,
                    Vector2::init(27, 17),
                    &mut known_weathers[3],
                    1,
                    ((beat - 1080) as f64 * 2.2).max(0.0)
                );
            },
            None,
        ),
    ]);

    weather.set_data::<[Weather; 5]>("known_weathers", Weather::KNOWN_WEATHERS);
    weather
}

pub fn funding() -> Scene {
    Scene::init("funding", vec![
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<Vec<Vec<String>>>("text", Vec::new());
                generator.set_data::<usize>("offset", 0);
                generator.set_data::<usize>("line_no", 0);
                generator.set_data::<String>("type_color", ansi_code(Fore::White, Style::Bright));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let type_color: String = generator.get_data::<String>("type_color")
                    .expect("type_color not found").clone();
                typewrite_by_word(
                    context.canvas,
                    context.manager_data,
                    generator,
                    Vector2::init(2, 22),
                    &type_color,
                    true,
                    "history"
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            None,
            |_, _, context: &mut Context<'_>| {
                write_history(
                    context.canvas,
                    context.manager_data,
                    Vector2::init(1, 19),
                    1,
                    "history"
                );
            },
            None,
        ),
    ])
}

pub fn loading() -> Scene {
    Scene::init("loadingbar", vec![
        Generator::init(
            0,
            Condition::At(0),
            Some(|_, context: &mut Context<'_>| {
                context.scene_data.set::<usize>("progress", 0);
            }),
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(2, 9),
                    "----------------------------------\n|                                |\n----------------------------------",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            1,
            Condition::At(1),
            Some(|_, context: &mut Context<'_>| {
                context.scene_data.set::<usize>("progress", 0);
            }),
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(3, 10),
                    "          Loading...          \n",
                    &ansi_code(Fore::Black, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::All(vec![Condition::Before(240), Condition::Every(8)]),
            None,
            |_, _, context: &mut Context<'_>| {
                *context.scene_data.get_mut::<usize>("progress").expect("data not found or type mismatch") += 1;
            },
            None,
        ),
        Generator::init(
            240,
            Condition::Always,
            None,
            |_, _, context: &mut Context<'_>| {
                let mut rng: ThreadRng = rand::rng();
                let increment: usize = rng.random_range::<i32, _>(40..=70) as usize;
                *context.scene_data.get_mut::<usize>("progress").expect("data not found or type mismatch") += increment;
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Before(240),
            None,
            |_, _, context: &mut Context<'_>| {
                let progress: usize = *context.scene_data.get::<usize>("progress")
                    .expect("progress not found");
                context.canvas.set_string(
                    Vector2::init(3, 10),
                    &"#".repeat(progress),
                    &ansi_code(Fore::Yellow, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            240,
            Condition::Always,
            None,
            |_, _, context: &mut Context<'_>| {
                let progress: usize = *context.scene_data.get::<usize>("progress")
                    .expect("progress not found");
                context.canvas.set_string(
                    Vector2::init(3, 10),
                    &fuck_up_text(&"#".repeat(progress), 400.0, ""),
                    &ansi_code(Fore::Red, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Before(240),
            Some(|generator: &mut Generator, _| {
                generator.set_data::<Vec<Vec<String>>>("text", Vec::new());
                generator.set_data::<usize>("offset", 0);
                generator.set_data::<usize>("line_no", 0);
                generator.set_data::<String>("type_color", ansi_code(Fore::Black, Style::Bright));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let type_color: String = generator.get_data::<String>("type_color")
                    .expect("type_color not found").clone();
                typewrite_by_word(
                    context.canvas,
                    context.manager_data,
                    generator,
                    Vector2::init(3, 12),
                    &type_color,
                    true,
                    "history"
                );
            },
            None,
        ),
    ])
}

pub fn quick_loading() -> Scene {
    Scene::init("fastload", vec![
        Generator::init(
            0,
            Condition::At(0),
            Some(|_, context: &mut Context<'_>| {
                context.scene_data.set::<usize>("progress", 0);
            }),
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(2, 9),
                    "----------------------------------\n|                                |\n----------------------------------",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            1,
            Condition::At(1),
            Some(|_, context: &mut Context<'_>| {
                context.scene_data.set::<usize>("progress", 0);
            }),
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(3, 10),
                    "        Please wait...        \n",
                    &ansi_code(Fore::Black, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            None,
            |_, _, context: &mut Context<'_>| {
                let mut rng: ThreadRng = rand::rng();
                let increment: usize = rng.random_range::<i32, _>(4..=6) as usize;
                *context.scene_data.get_mut::<usize>("progress").expect("data not found or type mismatch") += increment;
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            None,
            |_, _, context: &mut Context<'_>| {
                let progress: usize = *context.scene_data.get::<usize>("progress")
                    .expect("progress not found");
                context.canvas.set_string(
                    Vector2::init(3, 10),
                    &"#".repeat(progress),
                    &ansi_code(Fore::Green, Style::Bright)
                );
            },
            None,
        ),
    ])
}

pub fn error_screen() -> Scene {
    Scene::init("error", vec![
        Generator::init(
            0,
            Condition::At(0),
            Some(|_, context: &mut Context<'_>| {
                context.scene_data.set::<usize>("progress", 0);
            }),
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(0, 14),
                    "--------------------------------------------------------------------------------\n\n\n\n\n\n\n\n  $ ",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::At(0),
            Some(|_, context: &mut Context<'_>| {
                context.scene_data.set::<usize>("progress", 0);
            }),
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(1, 1),
                    "Automatic diagnosis unsuccessful. Please wait.\n> ",
                    &ansi_code(Fore::Black, Style::Bright)
                );
            },
            None,
        ),
    ])
}

pub fn funding_double() -> Scene {
    Scene::init("fundingx2", vec![
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<Vec<Vec<String>>>("text", Vec::new());
                generator.set_data::<usize>("offset", 0);
                generator.set_data::<usize>("line_no", 0);
                generator.set_data::<String>("type_color", ansi_code(Fore::Red, Style::Normal));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let type_color: String = generator.get_data::<String>("type_color")
                    .expect("type_color not found").clone();
                typewrite_by_word(
                    context.canvas,
                    context.manager_data,
                    generator,
                    Vector2::init(2, 2),
                    &type_color,
                    true,
                    "history"
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<Vec<Vec<String>>>("text", Vec::new());
                generator.set_data::<usize>("offset", 0);
                generator.set_data::<usize>("line_no", 0);
                generator.set_data::<String>("type_color", ansi_code(Fore::Cyan, Style::Bright));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let type_color: String = generator.get_data::<String>("type_color")
                    .expect("type_color not found").clone();
                typewrite_by_word(
                    context.canvas,
                    context.manager_data,
                    generator,
                    Vector2::init(2, 22),
                    &type_color,
                    true,
                    "history2"
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            None,
            |_, _, context: &mut Context<'_>| {
                write_history(
                    context.canvas,
                    context.manager_data,
                    Vector2::init(1, 21),
                    15,
                    "history2"
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<Vec<Vec<String>>>("text", Vec::new());
                generator.set_data::<usize>("offset", 0);
                generator.set_data::<usize>("line_no", 0);
                generator.set_data::<String>("type_color", ansi_code(Fore::Yellow, Style::Bright));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let type_color: String = generator.get_data::<String>("type_color")
                    .expect("type_color not found").clone();
                typewrite_by_word(
                    context.canvas,
                    context.manager_data,
                    generator,
                    Vector2::init(4, 5),
                    &type_color,
                    true,
                    "history3"
                );
            },
            None,
        ),
    ])
}

pub fn access_points() -> Scene {
    Scene::init("accesspoints", vec![
        Generator::init(
            0,
            Condition::All(vec![Condition::Before(20), Condition::Every(2)]),
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                let mut rng: ThreadRng = rand::rng();
                let string: String = (0..4)
                    .map(|yblock: usize| (0..3)
                        .map(|yline: usize| (0..6)
                            .map(|x: usize| {
                                if rng.random_range::<i32, _>(0..=(32 - (beat as f64).powf(1.2) as i32).max(1)) >= 4 {
                                    return "       ".to_string();
                                }
                                if yline == 1 { format!("PBS #{:02}", yblock * 6 + x + 1) }
                                else { ["  ###  ", "", "Unknown"][yline].to_string() }
                            })
                            .collect::<Vec<String>>().join("   "))
                        .collect::<Vec<String>>().join("\n"))
                    .collect::<Vec<String>>().join("\n\n");

                set_multiline_string(
                    context.canvas,
                    Vector2::init(1, 1),
                    &string,
                    &ansi_code(Fore::Black, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            20,
            Condition::At(20),
            None,
            |_, _, context: &mut Context<'_>| {
                let string: String = (0..4)
                    .map(|yblock: usize| (0..3)
                        .map(|yline: usize| (0..6)
                            .map(|x: usize| {
                                if yline == 1 { format!("PBS #{:02}", yblock * 6 + x + 1) }
                                else { ["  ###  ", "", "Unknown"][yline].to_string() }
                            })
                            .collect::<Vec<String>>().join("   "))
                        .collect::<Vec<String>>().join("\n"))
                    .collect::<Vec<String>>().join("\n\n");

                set_multiline_string(
                    context.canvas,
                    Vector2::init(1, 1),
                    &string,
                    &ansi_code(Fore::Red, Style::Normal)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::At(0),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(1, 17),
                    "No access points are broadcasting.\nManual search in progress.\nLast search 27.02.2019 (532 days ago)",
                    &ansi_code(Fore::Black, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            80,
            Condition::All(vec![Condition::Every(8), Condition::Before(976)]),
            Some(|generator: &mut Generator, _| {
                generator.set_data::<usize>("counter", 0);
                generator.set_data::<usize>("block", 0);
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                show_access_point_visual(
                    context.canvas,
                    generator,
                    Vector2::init(1, 1)
                );
            },
            None,
        ),
        Generator::init(
            968,
            Condition::At(968),
            None,
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(6, 9),
                    "  @@@  \nPBS #14\n Active",
                    &ansi_code(Fore::Green, Style::Bright)
                );
            },
            None,
        ),
    ])
}

pub fn funding_single() -> Scene {
    Scene::init("fdg_single", vec![
        Generator::init(
            0,
            Condition::At(0),
            Some(|_, context: &mut Context<'_>| {
                context.scene_data.set::<usize>("progress", 0);
            }),
            |_, _, context: &mut Context<'_>| {
                set_multiline_string(
                    context.canvas,
                    Vector2::init(0, 20),
                    "--------------------------------------------------------------------------------\n  Sending > ",
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<Vec<Vec<String>>>("text", Vec::new());
                generator.set_data::<usize>("offset", 0);
                generator.set_data::<usize>("line_no", 0);
                generator.set_data::<String>("type_color", ansi_code(Fore::Yellow, Style::Normal));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let type_color: String = generator.get_data::<String>("type_color")
                    .expect("type_color not found").clone();
                typewrite_by_word(
                    context.canvas,
                    context.manager_data,
                    generator,
                    Vector2::init(6, 21),
                    &type_color,
                    true,
                    "history"
                );
            },
            None,
        ),
    ])
}

pub fn funding_down() -> Scene {
    Scene::init("fdg_down", vec![
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<Vec<Vec<String>>>("text", Vec::new());
                generator.set_data::<usize>("offset", 0);
                generator.set_data::<usize>("line_no", 0);
                generator.set_data::<String>("type_color", ansi_code(Fore::Yellow, Style::Normal));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let line_no: usize = *generator.get_data::<usize>("line_no")
                    .expect("line_no not found");
                let type_color: String = generator.get_data::<String>("type_color")
                    .expect("type_color not found").clone();
                typewrite_by_word(
                    context.canvas,
                    context.manager_data,
                    generator,
                    Vector2::init(1, 1 + line_no),
                    &type_color,
                    true,
                    "history"
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            Some(|generator: &mut Generator, _| {
                generator.set_data::<Vec<Vec<String>>>("text", Vec::new());
                generator.set_data::<usize>("offset", 0);
                generator.set_data::<usize>("line_no", 0);
                generator.set_data::<String>("type_color", ansi_code(Fore::Yellow, Style::Normal));
            }),
            |generator: &mut Generator, _, context: &mut Context<'_>| {
                let line_no: usize = *generator.get_data::<usize>("line_no")
                    .expect("line_no not found");
                let type_color: String = generator.get_data::<String>("type_color")
                    .expect("type_color not found").clone();
                typewrite_by_word(
                    context.canvas,
                    context.manager_data,
                    generator,
                    Vector2::init(1, 20 + line_no),
                    &type_color,
                    true,
                    "history"
                );
            },
            None,
        ),
    ])
}

pub fn poweroff() -> Scene {
    Scene::init("poweroff", vec![
        Generator::init(
            3,
            Condition::Always,
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                make_poweroff_bars(
                    context.canvas,
                    beat - 2,
                    &ansi_code(Fore::Black, Style::Normal)
                );
            },
            None,
        ),
        Generator::init(
            2,
            Condition::Always,
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                make_poweroff_bars(
                    context.canvas,
                    beat - 1,
                    &ansi_code(Fore::Black, Style::Bright)
                );
            },
            None,
        ),
        Generator::init(
            1,
            Condition::Always,
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                make_poweroff_bars(
                    context.canvas,
                    beat,
                    &ansi_code(Fore::White, Style::Normal)
                );
            },
            None,
        ),
        Generator::init(
            0,
            Condition::Always,
            None,
            |_, beat: i32, context: &mut Context<'_>| {
                make_poweroff_bars(
                    context.canvas,
                    beat + 1,
                    &ansi_code(Fore::White, Style::Bright)
                );
            },
            None,
        ),
    ])
}

// beats_side is ignored according to the original codes.
pub fn all_scenes() -> Vec<Scene> {
    vec![
        wipe(),
        clear_wipe(),
        clear_scene(),
        ocean(),
        ocean2(),
        ocean3(),
        text(),
        title(),
        beats(),
        beats_lr(),
        funding(),
        date_ticker(),
        weather(),
        redraw_ui(),
        loading(),
        quick_loading(),
        error_screen(),
        funding_double(),
        access_points(),
        funding_single(),
        funding_down(),
        poweroff(),
    ]
}
