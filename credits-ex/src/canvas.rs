pub struct Canvas {
    pub dimensions: Vector2,
    pub layer: Layer,
    pub edits_this_frame: usize,
}

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: usize,
    pub y: usize,
}

pub struct Layer {
    pub sections: Vec<RenderSection>,
    pub full_str: Vec<String>
}

pub struct RenderSection {
    pub string: String,
    pub start: usize,
    pub length: usize,
    pub end: usize,
    pub code: String,
}

pub const UNIT_SIZE: Vector2 = Vector2 {
    x: 40,
    y: 24,
};

impl Canvas {

    pub fn new() -> Self {
        Self {
            dimensions: Vector2::new(),
            layer: Layer::new(),
            edits_this_frame: 0,
        }
    }

    pub fn init(dimensions: Vector2) -> Self {
        let dimensions: Vector2 = Vector2::init(
            dimensions.x * 2,
            dimensions.y,
        );
        Self {
            dimensions,
            layer: Layer::init(dimensions),
            ..Self::new()
        }
    }

    pub fn set_char(&mut self, position: Vector2, char: &str, code: &str) {
        self.layer.set_char(position.x * 2 + position.y * self.dimensions.x, char, code);
        self.edits_this_frame += 1;
    }

    pub fn set_string(&mut self, position: Vector2, string: &str, code: &str) {
        self.layer.set_string(position.x * 2 + position.y * self.dimensions.x, string, code);
        self.edits_this_frame += 1;
    }

    pub fn clear_layer(&mut self) {
        self.layer.set_string(0, &" ".repeat(self.dimensions.x * self.dimensions.y), "");
    }

    pub fn render_blank(&mut self) {
        println!("{}{}", "\x1b[1;1H\x1b[1;37;40m", ("  ".repeat(self.dimensions.x) + "\n").repeat(self.dimensions.y))
    }

    pub fn render_all(&mut self) {
        let mut total_string: String = "\x1b[1;1H\x1b[1;39m".to_string();
        for section in &self.layer.sections {
            let mut line_no: usize = section.start / self.dimensions.x;
            let mut add_string: String = String::new();
            add_string += &format!("\x1b[{};{}H", line_no + 1, section.start % self.dimensions.x + 1);

            add_string += &section.code;

            let mut offset: usize = section.start % self.dimensions.x;
            let mut split: usize = 0;
            let mut stop: usize = 0;
            let mut chars: std::str::Chars<'_> = section.string.chars();
            while stop <= section.length {
                if line_no >= self.dimensions.y { break }

                stop = self.dimensions.x - offset + split;
                add_string.extend(chars.by_ref().take(stop - split));
                add_string += if stop <= section.length { "\n" } else { "" };
            
                offset = 0;
                line_no += 1;
                split = stop;
            }
            total_string += &add_string
        }

        self.layer.sections.clear();
        self.edits_this_frame = 0;
        println!("{}", total_string + "\x1b[27;0H");
    }

}

impl Vector2 {

    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    pub fn init(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn offset(&self, dx: isize, dy: isize) -> Self {
        Self {
            x: (self.x as isize + dx) as usize,
            y: (self.y as isize + dy) as usize,
        }
    }

}

impl Layer {

    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            full_str: Vec::new(),
        }
    }

    pub fn init(dimensions: Vector2) -> Self {
        Self {
            // For an unknown reason, the original Layer doubles dimensions.x again after
            // Canvas has already doubled it, producing Layer dimensions of (160, 24).
            // full_str therefore includes another factor of two to match the original.
            full_str: vec![String::new(); dimensions.x * 2 * dimensions.y],
            ..Self::new()
        }
    }

    pub fn set_char(&mut self, location: usize, char: &str, code: &str) {
        let mut index: Option<usize> = self.bisect_right(location).checked_sub(1);
        if let Some(mut i) = index {
            while i >= 1 && self.sections[i - 1].start == location {
                i -= 1;
            }
            index = Some(i);
        }

        if let Some(i) = index {
            if self.sections[i].end >= location {
                if self.sections[i].code != code {
                    let section: &RenderSection = &self.sections[i];
                    let mut sub_sections: Vec<RenderSection> = Vec::new();

                    if location > section.start {
                        let string: String = section.string.chars()
                            .take(location - section.start).collect::<String>();
                        let length: usize = string.chars().count();
                        sub_sections.push(RenderSection {
                            string,
                            start: section.start,
                            length,
                            end: section.start + length,
                            code: section.code.clone(),
                        });
                    }
                    if location < section.end {
                        let string: String = section.string.chars()
                            .skip(section.end - location).collect::<String>();
                        let length: usize = string.chars().count();
                        sub_sections.push(RenderSection {
                            string,
                            start: location + 2,
                            length,
                            end: location + 2 + length,
                            code: section.code.clone(),
                        });
                    }

                    self.sections.remove(i);
                    for section in sub_sections {
                        let insert_i: usize = self.bisect_right(section.start);
                        self.sections.insert(insert_i, section);
                    }
                    index = None;
                } else {
                    let section: &mut RenderSection = &mut self.sections[i];
                    if location == section.end {
                        section.string += char;
                        section.length += 2;
                        section.end += 2;
                    } else {
                        section.string = format!("{}{}{}",
                            section.string.chars().take(location).collect::<String>(),
                            char,
                            section.string.chars().skip(location + 2).collect::<String>(),
                        );
                    }
                }
            } else {
                index = None;
            }
        }

        if index.is_none() {
            let length: usize = char.chars().count();
            let insert_i: usize = self.bisect_right(location);
            self.sections.insert(insert_i, RenderSection {
                string: char.to_string(),
                start: location,
                length,
                end: location + length,
                code: code.to_string(),
            });
        }

        self.full_str[location] = code.to_string() + char;
    }

    pub fn set_string(&mut self, location: usize, string: &str, code: &str) {
        if location >= self.full_str.len() { return }

        let mut start_i: usize = self.bisect_right(location).saturating_sub(1);
        while start_i >= 1 && self.sections[start_i - 1].start == location {
            start_i -= 1;
        }

        let cut_string: String = string.chars().take(self.full_str.len() - location).collect::<String>();
        let length: usize = cut_string.chars().count();
        let mut add_group: RenderSection = RenderSection {
            string: cut_string,
            start: location,
            length,
            end: location + length,
            code: code.to_string(),
        };
        let (mut first, mut last): (Option<RenderSection>, Option<RenderSection>) = (None, None);
        let mut remove: Vec<usize> = Vec::new();

        for mid_i in start_i..self.sections.len() {
            let mid_group: &RenderSection = &self.sections[mid_i];
            if mid_group.start >= add_group.end { break }

            if mid_group.end > add_group.start {
                if mid_group.start >= add_group.start && mid_group.end <= add_group.end {
                    remove.push(mid_i);
                } 
                else if mid_group.code == add_group.code {
                    let diff: isize = add_group.start as isize - mid_group.start as isize;
                    let count: isize = mid_group.string.chars().count() as isize;
                    let prefix: isize = if diff < 0 { (count + diff).max(0) } else { diff.min(count) };
                    let tail: isize = diff + add_group.length as isize;
                    let suffix: isize = if tail < 0 { (count + tail).max(0) } else { tail.min(count) };

                    add_group.string = format!("{}{}{}",
                        mid_group.string.chars().take(prefix as usize).collect::<String>(),
                        add_group.string,
                        mid_group.string.chars().skip(suffix as usize).collect::<String>(),
                    );
                    add_group.start = add_group.start.min(mid_group.start);
                    add_group.length = add_group.string.chars().count();
                    add_group.end = add_group.start + add_group.length;
                    remove.push(mid_i);
                } 
                else {
                    if add_group.start > mid_group.start {
                        let string: String = mid_group.string.chars()
                            .take(add_group.start - mid_group.start).collect::<String>();
                        let length: usize = string.chars().count();
                        first = Some(RenderSection {
                            string,
                            start: mid_group.start,
                            length,
                            end: mid_group.start + length,
                            code: mid_group.code.clone(),
                        });
                    }
                    if add_group.end < mid_group.end {
                        let diff: usize = mid_group.end - add_group.end;
                        let string: String = mid_group.string.chars()
                            .skip(mid_group.length - diff).collect::<String>();
                        let length: usize = string.chars().count();
                        last = Some(RenderSection {
                            string,
                            start: add_group.end,
                            length,
                            end: add_group.end + length,
                            code: mid_group.code.clone(),
                        });
                    }
                    remove.push(mid_i);
                }
            }
        }

        for index in remove.into_iter().rev() {
            self.sections.remove(index);
        }

        let mut add_index: usize = self.bisect_right(location);
        for section in [first, Some(add_group), last] {
            if let Some(section) = section {
                let index: usize = (start_i + add_index).min(self.sections.len());
                self.sections.insert(index, section);
                add_index += 1;
            }
        }

        for (index, char) in string.chars().enumerate() {
            self.full_str[index] = format!("{}{}", code, char);
        }
    }

    fn bisect_right(&self, location: usize) -> usize {
        let (mut low, mut high): (usize, usize) = (0, self.sections.len());
        while low < high {
            let mid: usize = low + (high - low) / 2;
            if location < self.sections[mid].start {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}

impl RenderSection {

    pub fn new() -> Self {
        Self {
            string: String::new(),
            start: 0,
            length: 0,
            end: 0,
            code: String::new(),
        }
    }

    pub fn init(string: &str, start: usize, code: &str) -> Self {
        let length: usize = string.chars().count();
        Self {
            string: string.to_string(),
            start,
            length,
            end: start + length,
            code: code.to_string(),
        }
    }

    pub fn update(&mut self) {
        self.length = self.string.chars().count();
        self.end = self.start + self.length;
    }

    pub fn add_char(&mut self, char: &str) {
        self.string += char;
        self.length += 2;
        self.end += 2;
    }

    pub fn modify_char(&mut self, char: &str, location: usize) {
        match location == self.end {
            true => self.add_char(char),
            false => self.string = format!("{}{}{}", 
                self.string.chars().take(location).collect::<String>(), 
                char, 
                self.string.chars().skip(location + 2).collect::<String>(),
            )
        }
    }

    pub fn add_section(&mut self, other: RenderSection) {
         let diff: isize = other.start as isize - self.start as isize;
        let count: isize = self.string.chars().count() as isize;
        let prefix: isize = if diff < 0 { (count + diff).max(0) } else { diff.min(count) };
        let tail: isize = diff + other.length as isize;
        let suffix: isize = if tail < 0 { (count + tail).max(0) } else { tail.min(count) };

        self.string = format!("{}{}{}",
            self.string.chars().take(prefix as usize).collect::<String>(),
            other.string,
            self.string.chars().skip(suffix as usize).collect::<String>(),
        );
        self.start = self.start.min(other.start);
        self.update();
    }

    pub fn add_section_below(&mut self, other: RenderSection) {
        let diff: isize = self.start as isize - other.start as isize;
        let count: isize = other.string.chars().count() as isize;
        let prefix: isize = if diff < 0 { (count + diff).max(0) } else { diff.min(count) };
        let tail: isize = diff + self.length as isize;
        let suffix: isize = if tail < 0 { (count + tail).max(0) } else { tail.min(count) };

        self.string = format!("{}{}{}",
            other.string.chars().take(prefix as usize).collect::<String>(),
            self.string,
            other.string.chars().skip(suffix as usize).collect::<String>(),
        );
        self.start = other.start.min(self.start);
        self.update();
    }

    pub fn substract_char(&mut self, location: usize) -> (Option<RenderSection>, Option<RenderSection>) {
        let (mut first, mut last): (Option<RenderSection>, Option<RenderSection>) = (None, None);
        
        if location > self.start {
            let diff: usize = location - self.start;
            first = Some(Self::init(
                &self.string.chars().take(diff).collect::<String>(),
                self.start,
                &self.code,
            ))
        }
        if location < self.end {
            let diff: usize = self.end - location;
            last = Some(Self::init(
                &self.string.chars().skip(diff).collect::<String>(),
                location + 2,
                &self.code
            ))
        }
        (first, last)
    }

    pub fn substract_section(&mut self, other: RenderSection) -> (Option<RenderSection>, Option<RenderSection>) {
        let (mut first, mut last): (Option<RenderSection>, Option<RenderSection>) = (None, None);

        if other.start > self.start {
            let diff: usize = other.start - self.start;
            first = Some(Self::init(
                &self.string.chars().take(diff).collect::<String>(),
                self.start,
                &self.code,
            ))
        }
        if other.end < self.end {
            let diff: usize = self.end - other.end;
            let count: isize = self.string.chars().count() as isize;
            let tail: isize = self.length as isize - diff as isize;
            let suffix: isize = if tail < 0 { (count + tail).max(0) } else { tail.min(count) };

            last = Some(Self::init(
                &self.string.chars().skip(suffix as usize).collect::<String>(),
                other.end,
                &self.code
            ))
        }
        (first, last)
    }
}