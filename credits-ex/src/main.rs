mod animation;
mod broadcast;
mod canvas;
mod color;
mod terminal;
mod weather;

use std::{env, error::Error, fs::File, io::{self, BufReader, Write}, path::PathBuf};
use rand::{RngExt, seq::IndexedRandom};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};

use crate::animation::engine::{Action, DataStore, Event, Manager};
use crate::animation::scenes::all_scenes;
use crate::broadcast::broadcasts;
use crate::canvas::UNIT_SIZE;
use crate::color::{Fore, Style, ansi_code};
use crate::terminal::{Cursor, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let executable: PathBuf = env::current_exe()?;
    let filename: PathBuf = executable.parent().ok_or_else(|| io::Error::other("EXE directory not found"))?.join("audio").join("credits-ex.wav");
    let file: File = File::open(&filename).map_err(|error: io::Error| io::Error::new(error.kind(), format!("Open audio '{}': {error}", filename.display())))?;

    let Some(terminal): Option<Terminal> = Terminal::init(&executable)? else {
        return Ok(());
    };

    let ocean_time: usize = (rand::rng().random::<f64>() * 2000.0).floor() as usize;
    let broadcasts: DataStore = broadcasts();

    let ocean_events: Vec<Event> = vec![
        Event::init(60, Action::Action(|controller: &mut Manager| {
            let text: String = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<String>("ocean_b_0").expect("ocean_b_0 not found").clone();
            controller.set_generator_data::<String>("ocean_b", 1, "text", text);
        })),
        Event::init(310, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 1, "text", "[##CLEAR|60;6".to_string());
        })),
        Event::init(310, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<f64>("ocean_b", 0, "ocean_glitch", 3.0);
        })),
        Event::init(310, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 0, "ocean_col", ansi_code(Fore::Blue, Style::Normal));
        })),
        Event::init(312, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 0, "ocean_col", ansi_code(Fore::Blue, Style::Bright));
        })),
        Event::init(314, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 0, "ocean_col", ansi_code(Fore::Blue, Style::Normal));
        })),
        Event::init(320, Action::Action(|controller: &mut Manager| {
            let text: String = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<String>("ocean_b_1").expect("ocean_b_1 not found").clone();
            controller.set_generator_data::<String>("ocean_b", 1, "text", text);
            controller.set_generator_data::<usize>("ocean_b", 1, "offset", 0);
        })),
        Event::init(590, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 0, "ocean_col", ansi_code(Fore::Black, Style::Bright));
        })),
        Event::init(646, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 1, "text", "[##CLEAR|60;8".to_string());
        })),
        Event::init(652, Action::Action(|controller: &mut Manager| {
            let text: String = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<String>("ocean_b_2").expect("ocean_b_2 not found").clone();
            controller.set_generator_data::<String>("ocean_b", 1, "text", text);
            controller.set_generator_data::<usize>("ocean_b", 1, "offset", 0);
        })),
        Event::init(656, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 0, "ocean_col", ansi_code(Fore::Yellow, Style::Normal));
        })),
        Event::init(666, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 0, "ocean_col", ansi_code(Fore::Yellow, Style::Bright));
        })),
        Event::init(666, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<f64>("ocean_b", 0, "ocean_glitch", 6.0);
        })),
        Event::init(850, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 0, "ocean_col", ansi_code(Fore::Yellow, Style::Normal));
        })),
        Event::init(999, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 1, "text", "[##CLEAR|60;10".to_string());
        })),
        Event::init(1000, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_b", 0, "ocean_col", ansi_code(Fore::Black, Style::Bright));
        })),
        Event::init(1000, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<f64>("ocean_b", 0, "ocean_glitch", 13.0);
        })),
        Event::init(1000, Action::Action(|controller: &mut Manager| {
            let text: String = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<String>("ocean_b_3").expect("ocean_b_3 not found").clone();
            controller.set_generator_data::<String>("ocean_b", 1, "text", text);
            controller.set_generator_data::<usize>("ocean_b", 1, "offset", 0);
        })),
        Event::init(1044, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<f64>("ocean_b", 0, "ocean_glitch", 102.0);
        })),
        Event::init(1048, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<f64>("ocean_b", 0, "ocean_glitch", 230.0);
        })),
        Event::init(1052, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<f64>("ocean_b", 0, "ocean_glitch", 500.0);
        })),
        Event::init(1056, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<f64>("ocean_b", 0, "ocean_glitch", 760.0);
        })),
        Event::init(1060, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<f64>("ocean_b", 0, "ocean_glitch", 1600.0);
        })),
        Event::init(1064, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<f64>("ocean_b", 0, "ocean_glitch", 2500.0);
        })),
    ];

    let mut ocean2_events: Vec<Event> = vec![
        Event::init(3896, Action::Action(|controller: &mut Manager| {
            let text: String = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<String>("ocean_c_0").expect("ocean_c_0 not found").clone();
            controller.set_generator_data::<String>("ocean_c", 1, "text", text);
        })),
        Event::init(3896, Action::Action(|controller: &mut Manager| {
            let text: String = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<String>("ocean_c_1").expect("ocean_c_1 not found").clone();
            controller.set_generator_data::<String>("ocean_c", 2, "text", text);
        })),
        Event::init(3896, Action::Action(|controller: &mut Manager| {
            let text: String = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<String>("ocean_c_2").expect("ocean_c_2 not found").clone();
            controller.set_generator_data::<String>("ocean_c", 3, "text", text);
        })),
        Event::init(3896, Action::Action(|controller: &mut Manager| {
            let text: String = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<String>("ocean_c_3").expect("ocean_c_3 not found").clone();
            controller.set_generator_data::<String>("ocean_c", 4, "text", text);
        })),
        Event::init(4400, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<String>("ocean_c", 1, "text", "[##CLEAR|60;10".to_string());
        })),
    ];

    for i in 0..5 {
        ocean2_events.push(Event::init(4401 + i * 2, Action::Action(|controller: &mut Manager| {
            let colors: [String; 2] = [ansi_code(Fore::Black, Style::Bright), ansi_code(Fore::Black, Style::Normal)];
            let color: String = colors.choose(&mut rand::rng()).expect("color not found").clone();
            controller.set_generator_data::<String>("ocean_c", 0, "ocean_col", color);
        })));
    }

    ocean2_events.push(Event::init(4411, Action::Action(|controller: &mut Manager| {
        controller.set_generator_data::<String>("ocean_c", 0, "ocean_col", ansi_code(Fore::Black, Style::Normal));
    })));

    let mut events: Vec<Event> = vec![
        Event::init(0, Event::swap_scene("wipe", 0)),
        Event::init(58, Event::swap_scene("clear", 0)),
        Event::init(60, Event::swap_scene("ocean_b", 0)),
    ];
    events.extend(ocean_events);
    events.extend(vec![
        Event::init(1079, Event::swap_scene("clear", 0)),
        Event::init(1080, Event::swap_scene("beats", 0)),
        Event::init(1080, Event::layer_scene("title", 0)),
        Event::init(1080, Event::swap_scene("beats", 0)),
        Event::init(1336, Event::swap_scene("beats_lr", 0)),
        Event::init(1844, Event::remove_scene("title")),
        Event::init(1848, Event::swap_scene("funding", 0)),
        Event::init(1848, Event::layer_scene("dates", 0)),
        Event::init(1848, Event::layer_scene("weather", 0)),
        Event::init(1848, Action::Action(|controller: &mut Manager| {
            controller.set_data::<Vec<(String, String)>>("history", Vec::new());
            controller.set_data::<bool>("refresh", true);
        })),
        Event::init(1848, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("funding_0").expect("funding_0 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("funding", 0, "text", text);
        })),
        Event::init(2348, Action::Action(|controller: &mut Manager| {
            controller.set_generator_data::<Vec<Vec<String>>>("funding", 0, "text", vec![vec![String::new()], vec![String::new()]]);
        })),
        Event::init(2348, Action::Action(|controller: &mut Manager| {
            controller.set_data::<Vec<(String, String)>>("history", Vec::new());
            controller.set_data::<bool>("refresh", true);
        })),
        Event::init(2352, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("funding_1").expect("funding_1 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("funding", 0, "text", text);
            controller.set_generator_data::<usize>("funding", 0, "offset", 0);
            controller.set_generator_data::<usize>("funding", 0, "line_no", 0);
        })),
        Event::init(2976, Event::remove_scene("dates")),
        Event::init(2976, Event::remove_scene("weather")),
        Event::init(2976, Event::swap_scene("clear_wipe", 0)),
        Event::init(3007, Event::swap_scene("clear", 0)),
        Event::init(3132, Event::swap_scene("loadingbar", 0)),
        Event::init(3132, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("funding_2").expect("funding_2 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("loadingbar", 6, "text", text);
        })),
        Event::init(3376, Event::swap_scene("ocean_d", 0)),
        Event::init(3380, Event::swap_scene("clear", 0)),
        Event::init(3380, Event::swap_scene("fastload", 0)),
        Event::init(3388, Event::swap_scene("clear", 0)),
        Event::init(3390, Event::swap_scene("error", 0)),
        Event::init(3390, Event::layer_scene("fundingx2", 0)),
        Event::init(3390, Action::Action(|controller: &mut Manager| {
            controller.set_data::<Vec<(String, String)>>("history", Vec::new());
            controller.set_data::<bool>("refresh", true);
        })),
        Event::init(3390, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("fundingx2_0").expect("fundingx2_0 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("fundingx2", 0, "text", text);
        })),
        Event::init(3390, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("fundingx2_1").expect("fundingx2_1 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("fundingx2", 1, "text", text);
        })),
        Event::init(3390, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("fundingx2_2").expect("fundingx2_2 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("fundingx2", 3, "text", text);
        })),
        Event::init(3895, Event::remove_scene("fundingx2")),
        Event::init(3895, Event::swap_scene("clear", 0)),
        Event::init(3896, Event::swap_scene("ocean_c", 0)),
    ]);
    events.extend(ocean2_events);
    events.extend(vec![
        Event::init(4413, Event::swap_scene("clear", 0)),
        Event::init(4460, Event::swap_scene("accesspoints", 0)),
        Event::init(4460, Event::layer_scene("fdg_single", 0)),
        Event::init(4534, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("fdg_single_0").expect("fdg_single_0 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("fdg_single", 1, "text", text);
        })),
        Event::init(5500, Event::remove_scene("fdg_single")),
        Event::init(5500, Event::swap_scene("clear", 0)),
        Event::init(5500, Event::swap_scene("beats", 0)),
        Event::init(5500, Event::layer_scene("fdg_down", 0)),
        Event::init(5788, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("fdg_down_0").expect("fdg_down_0 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("fdg_down", 0, "text", text);
        })),
        Event::init(5916, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("fdg_down_1").expect("fdg_down_1 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("fdg_down", 0, "text", text);
            controller.set_generator_data::<usize>("fdg_down", 0, "line_no", 0);
            controller.set_generator_data::<usize>("fdg_down", 0, "offset", 0);
            // Intentional change from the original Python: replace the upper text with dashes in place, preserving words until the advancing prefix covers them.
            controller.set_generator_data::<bool>("fdg_down", 0, "clear_line", false);
        })),
        Event::init(5916, Action::Action(|controller: &mut Manager| {
            let text: Vec<Vec<String>> = controller.get_data::<DataStore>("broadcasts").expect("broadcasts not found")
                .get::<Vec<Vec<String>>>("fdg_down_2").expect("fdg_down_2 not found").clone();
            controller.set_generator_data::<Vec<Vec<String>>>("fdg_down", 1, "text", text);
        })),
        Event::init(6270, Event::swap_scene("beats_lr", 0)),
        Event::init(6508, Event::remove_scene("fdg_down")),
        Event::init(6508, Event::swap_scene("clear", 0)),
    ]);

    let mut controller: Manager = Manager::init(all_scenes(), events, UNIT_SIZE);
    controller.set_data::<usize>("ocean_time", ocean_time);
    controller.set_data::<DataStore>("broadcasts", broadcasts);

    let delay: f64 = 60.0 / 179.0 / 8.0;
    let mut beat: i32 = -60;
    let offset: f64 = 5.492;

    let source: Decoder<BufReader<File>> = Decoder::try_from(file)?;
    let mut handle: MixerDeviceSink = DeviceSinkBuilder::open_default_sink()?;
    handle.log_on_drop(false);
    let playback: Player = Player::connect_new(handle.mixer());
    playback.pause();
    playback.append(source);

    let _cursor: Cursor = Cursor::init()?;
    if !terminal.ready()? {
        return Ok(());
    }
    playback.play();

    while !playback.empty() {
        let next_beat: bool = (playback.get_pos().as_secs_f64() - offset) > ((beat - 1) as f64 * delay);

        if next_beat {
            controller.request_next(true);
            controller.canvas.render_all();
            io::stdout().flush()?;
            beat += 1;
        }
    }

    terminal.clear()?;
    Ok(())
}
