use rand::{RngExt, prelude::ThreadRng};

pub struct Weather {
    pub date: Date,
    pub kind: WeatherKind,
    pub temperature: f64,
    pub wind: Wind,
    pub precipitation: f64,
    pub humidity: f64,
}

pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub days: u32,
}

#[derive(Clone, Copy)]
pub enum WeatherKind {
    Blizzard,
    Clear,
    Cloudy,
    Drizzle,
    Hurricane,
    Overcast,
    PartlyCloudy,
    PartlySunny,
    Rain,
    Sleet,
    Snow,
    Snowstorm,
    Storm,
    Sunny,
    Lost,
}

pub struct Wind {
    pub wind_mph: f64,
    pub gust_mph: f64,
    pub direction: WindDirection,
}

#[derive(Clone, Copy)]
pub enum WindDirection {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl WeatherKind {

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blizzard => "Blizzard",
            Self::Clear => "Clear",
            Self::Cloudy => "Cloudy",
            Self::Drizzle => "Drizzle",
            Self::Hurricane => "Hurricane",
            Self::Overcast => "Overcast",
            Self::PartlyCloudy => "Partly cloudy",
            Self::PartlySunny => "Partly sunny",
            Self::Rain => "Rain",
            Self::Sleet => "Sleet",
            Self::Snow => "Snow",
            Self::Snowstorm => "Snowstorm",
            Self::Storm => "Storm",
            Self::Sunny => "Sunny",
            Self::Lost => "Connection lost...      ",
        }
    }

}

impl WindDirection {
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::N  => "N ",
            Self::NE => "NE",
            Self::E  => "E ",
            Self::SE => "SE",
            Self::S  => "S ",
            Self::SW => "SW",
            Self::W  => "W ",
            Self::NW => "NW",
        }
    }
    
    pub fn rotate(self, steps: i32) -> Self {
        let directions: [WindDirection; 8] = [
            Self::N, 
            Self::NE, 
            Self::E, 
            Self::SE,
            Self::S, 
            Self::SW, 
            Self::W, 
            Self::NW,
        ];
        let index: usize = (self as i32 + steps).rem_euclid(8) as usize;

        directions[index]
    }

}

impl Weather {

    pub fn new() -> Self {
        Self {
            date: Date {
                year: 2026,
                month: 7,
                day: 15,
                days: 0,
            },
            kind: WeatherKind::Rain,
            temperature: 59.0,
            wind: Wind {
                wind_mph: 15.0,
                gust_mph: 25.0,
                direction: WindDirection::E,
            },
            precipitation: 0.55,
            humidity: 0.75,
        }
    }

    pub fn init(&mut self, precip: f64, temp: f64, wind: Wind, humi: f64, days: u32) {
        self.precipitation = precip;
        self.temperature = temp;
        self.wind = wind;
        self.humidity = humi;
        self.kind = self.get_weather_kind();
        self.date.days = days;
    }

    pub fn as_string(&self) -> String{
        format!(
            concat!(
                "{}% precipitation\n",
                "{}F temperature\n",
                "{}mph wind\n",
                "{}mph gust\n",
                "{} wind direction\n",
                "{}% humidity\n",
                "{}\n",
            ),
            (self.precipitation * 100.0 * 100.0).round() / 100.0,
            self.temperature as i32,
            self.wind.wind_mph as i32,
            self.wind.gust_mph as i32,
            self.wind.direction as usize,
            (self.humidity * 100.0 * 100.0).round() / 100.0,
            self.kind.as_str(),
        )
    }

    pub fn get_weather_kind(&self) -> WeatherKind {
        match (
            self.humidity,
            self.precipitation,
            self.wind.wind_mph,
            self.temperature,
        ) {
            (humi, precip, wind, temp) if humi > 0.5 && precip > 0.4 && wind > 43.0 && temp < 32.0 => WeatherKind::Blizzard,
            (humi, precip, wind, _)    if humi > 0.5 && precip > 0.4 && wind > 43.0                => WeatherKind::Hurricane,

            (humi, precip, wind, temp) if humi > 0.5 && precip > 0.4 && wind > 25.0 && temp < 32.0 => WeatherKind::Snowstorm,
            (humi, precip, wind, _)    if humi > 0.5 && precip > 0.4 && wind > 25.0                => WeatherKind::Storm,

            (humi, precip, _, temp)    if humi > 0.5 && precip > 0.4 && temp < 32.0                => WeatherKind::Snow,
            (humi, precip, _, _)       if humi > 0.5 && precip > 0.4                               => WeatherKind::Rain,

            (humi, precip, _, temp)    if humi > 0.5 && precip > 0.25 && temp < 32.0               => WeatherKind::Sleet,
            (humi, precip, _, _)       if humi > 0.5 && precip > 0.25                              => WeatherKind::Drizzle,

            // We have already handled precip above 0.25 up there, so why do we check this again?
            // Thinking...
            // Thinking harder...
            // The server is busy. Please try again later.
            (humi, precip, _, _)       if humi > 0.5 && (precip > 0.5 || humi > 0.8)               => WeatherKind::Overcast,
            (humi, precip, _, _)       if humi > 0.5 && (precip > 0.3 || humi > 0.65)              => WeatherKind::Cloudy,
            // Equivalent to:
            // (humi, _, _, _)         if humi > 0.8                                               => WeatherKind::Overcast,
            // (humi, _, _, _)         if humi > 0.65                                              => WeatherKind::Cloudy,

            (humi, _, _, _)            if humi > 0.5                                               => WeatherKind::PartlyCloudy,

            (_, precip, _, temp)       if precip > 0.4 && temp < 32.0                              => WeatherKind::Snow,
            (_, precip, _, _)          if precip > 0.4                                             => WeatherKind::Rain,

            (_, precip, _, temp)       if precip > 0.2 && temp < 32.0                              => WeatherKind::Sleet,
            (_, precip, _, _)          if precip > 0.2                                             => WeatherKind::Drizzle,

            (humi, _, _, _)            if humi < 0.1                                               => WeatherKind::Sunny,
            (humi, _, _, _)            if humi < 0.2                                               => WeatherKind::PartlySunny,

            _ => WeatherKind::Clear,
        }
    }

    pub fn mutate(&mut self, steps: u32) {
        let mut rng: ThreadRng = rand::rng();

        for _ in 0..steps {
            // Precipitation
            let mut precip_move: f64 = rng.random_range::<i32, _>(-100..=100) as f64 / 400.0;
            precip_move += (rng.random_range::<i32, _>(0..=((100.0 * (0.33 - self.precipitation).abs()) as i32)) as f64 / 200.0
            ) * if 0.33 - self.precipitation < 0.0 { -1.0 } else { 1.0 };
            self.precipitation = (self.precipitation + precip_move).clamp(0.0, 1.0);

            // Wind
            self.wind.direction = self.wind.direction.rotate(rng.random_range::<i32, _>(-3..=3));

            let mut wind_move: f64 = rng.random_range::<i32, _>(-100..=100) as f64 / 13.0;
            wind_move += (rng.random_range::<i32, _>(0..=((15.0 - self.wind.wind_mph).abs() as i32)) as f64 / 3.0
            ) * if 15.0 - self.wind.wind_mph < 0.0 { -1.0 } else { 1.0 };
            self.wind.wind_mph = (self.wind.wind_mph + wind_move).max(0.0);

            // Temperature
            let adjusted_days: f64 = self.date.days as f64 - 282.0;
            let temp_target: f64 = 40.0 * ((2.0 * std::f64::consts::PI * adjusted_days / 365.0 - std::f64::consts::PI / 3.0).sin() + 1.0) + 20.0;

            let mut temp_move: f64 = rng.random_range::<i32, _>(-100..=100) as f64 / 20.0;
            temp_move += (rng.random_range::<i32, _>(0..=((temp_target - self.temperature).abs() as i32)) as f64 / 5.0
            ) * if temp_target - self.temperature < 0.0 { -1.0 } else { 1.0 };
            self.temperature = (self.temperature + temp_move).max(0.0);

            // Humidity
            let mut humi_move: f64 = rng.random_range::<i32, _>(-100..=100) as f64 / 500.0;
            // In the original source code, the humidity bias is calculated using
            // "int(abs(0.2 - humidity))"", which always evaluates to 0 for humidity in [0, 1].
            // This behavior is intentionally preserved for consistency with the original.
            humi_move += (rng.random_range::<i32, _>(0..=((0.2 - self.humidity).abs() as i32)) as f64 / 200.0
            ) * if 0.5 - self.humidity < 0.0 { -1.0 } else { 1.0 };
            self.humidity = (self.humidity + humi_move).clamp(0.0, 1.0);

            // Others
            self.wind.gust_mph = self.wind.wind_mph * rng.random_range::<i32, _>(210..=260) as f64 * 0.01;

            self.date.days += 1;
        }

        self.kind = self.get_weather_kind();
    }

    pub const KNOWN_WEATHERS:[Weather; 5] =
    [
        // 22.10.2009: Cloudy
        Weather {
            date: Date {
                year: 2009,
                month: 10,
                day: 22,
                days: 2,
            },
            kind: WeatherKind::Cloudy,
            temperature: 43.0,
            wind: Wind {
                wind_mph: 13.0,
                gust_mph: 25.0,
                direction: WindDirection::W,
            },
            precipitation: 0.203,
            humidity: 0.66,
        },

        // 23.10.2009: Sunny
        Weather {
            date: Date {
                year: 2009,
                month: 10,
                day: 23,
                days: 2,
            },
            kind: WeatherKind::Sunny,
            temperature: 52.0,
            wind: Wind {
                wind_mph: 12.0,
                gust_mph: 25.0,
                direction: WindDirection::NW,
            },
            precipitation: 0.04,
            // Original humidity: 0.1, which yields "Partly sunny" instead of the forecasted "Sunny".
            // Adjusted to 0.09 for consistency.
            humidity: 0.09,
        },

        // 24.10.2009: Partly sunny
        Weather {
            date: Date {
                year: 2009,
                month: 10,
                day: 24,
                days: 200,
            },
            kind: WeatherKind::PartlySunny,
            temperature: 48.0,
            wind: Wind {
                wind_mph: 8.0,
                gust_mph: 20.0,
                direction: WindDirection::NE,
            },
            precipitation: 0.07,
            humidity: 0.1,
        },

        Weather {
            date: Date {
                year: 2011,
                month: 4,
                day: 5,
                days: 728,
            },
            kind: WeatherKind::PartlySunny,
            temperature: 48.0,
            wind: Wind {
                wind_mph: 8.0,
                gust_mph: 20.0,
                direction: WindDirection::NE,
            },
            precipitation: 0.07,
            humidity: 0.1,
        },

        // Connection lost...
        Weather {
            date: Date {
                year: 2099,
                month: 12,
                day: 31,
                days: 65536,
            },
            kind: WeatherKind::Lost,
            temperature: -1.0,
            wind: Wind {
                wind_mph: -1.0,
                gust_mph: -1.0,
                direction: WindDirection::NW,
            },
            precipitation: -1.0,
            humidity: -1.0,
        },
    ];
}


