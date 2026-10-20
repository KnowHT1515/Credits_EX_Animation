use rand::RngExt;

// ocean_time = (rand::rng().random::<f64>() * 2000.0).floor() as usize
pub const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn init_populate_ocean(ocean_time: &mut usize) -> Vec<String> {
    let ocean_base: Vec<String> = get_ocean_slices(*ocean_time, *ocean_time + 80);

    *ocean_time += 80;
    ocean_base
}

pub fn get_ocean_slice(xr: usize, glitch: f64) -> Vec<char> {
    let mut rng = rand::rng();
    let x: f64 = xr as f64 / 5.0;
    let c: f64 = (0.2 * x).cos() + (0.3 * x).sin() * (0.23 * x).sin();
    let y: i32 = -((2.0 * c * x.sin()).floor() as i32) + 3;
    let mut cont_list: Vec<char> = Vec::new();

    for i in 0..10 {
        if rng.random::<f64>() <= 0.002 * glitch {
            let index: usize = (rng.random::<f64>() * ALPHABET.len() as f64).floor() as usize;
            cont_list.push(ALPHABET.as_bytes()[index] as char);
        } 
        else if i == y {
            cont_list.push('#');
        } 
        else if i > y {
            cont_list.push('.');
        } 
        else {
            cont_list.push('\u{00a0}');
        }
    }

    cont_list
}

pub fn mutate_text(txt: &str, glitch: f64, ocean_time: usize) -> String {
    let mut rng = rand::rng();
    let mut txt: Vec<char> = txt.chars().collect::<Vec<char>>();

    for i in 0..txt.len() {
        let ch: char = txt[i];
        if "#.\u{00a0}".contains(ch)
            && rng.random::<f64>() <= (0.0002 + (ocean_time % 1200) as f64 / 1200000.0) * glitch
        {
            let index: usize = (rng.random::<f64>() * ALPHABET.len() as f64).floor() as usize;
            txt[i] = ALPHABET.as_bytes()[index] as char;
        }
    }

    txt.into_iter().collect::<String>()
}

pub fn get_ocean_slices(x1: usize, x2: usize) -> Vec<String> {
    let mut rng = rand::rng();
    let mut cont_list: Vec<String> = vec![String::new(); 10];

    for xr in x1..x2 {
        let x: f64 = xr as f64 / 5.0;
        let c: f64 = (0.2 * x).cos() + (0.3 * x).sin() * (0.23 * x).sin();
        let y: i32 = -((2.0 * c * x.sin()).floor() as i32) + 3;

        for i in 0..10 {
            if rng.random::<f64>() <= 0.002 {
                let index: usize = (rng.random::<f64>() * ALPHABET.len() as f64).floor() as usize;
                cont_list[i].push(ALPHABET.as_bytes()[index] as char);
            } else if i as i32 == y {
                cont_list[i].push('#');
            } else if i as i32 > y {
                cont_list[i].push('.');
            } else {
                cont_list[i].push('\u{00a0}');
            }
        }
    }

    cont_list
}

pub fn update_ocean_slices(ocean_content: &mut [String], ocean_glitch: f64, ocean_slice_glitch: f64, ocean_time: &mut usize) -> String {
    // Intentional change from the original Python: new slices can use a separate
    // glitch strength; ocean_glitch still controls mutation across the whole frame.
    let ocean_slice: Vec<char> = get_ocean_slice(*ocean_time, ocean_slice_glitch);
    *ocean_time += 1;

    for i in 0..10 {
        ocean_content[i] = format!("{}{}",
            ocean_content[i].chars().skip(1).collect::<String>(),
            ocean_slice[i],
        );
    }

    let raw_txt: String = unpack_content_to_text(ocean_content);
    mutate_text(&raw_txt, ocean_glitch, *ocean_time)
}

pub fn unpack_content_to_text(content: &[String]) -> String {
    content.concat()
}

pub fn begin_ocean(ocean_time: &mut usize) -> Vec<String> {
    init_populate_ocean(ocean_time)
}
