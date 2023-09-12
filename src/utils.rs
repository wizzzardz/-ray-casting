pub struct Config {
    pub screen_width: i32,
    pub screen_height: i32,
}

impl Config {
    pub fn new(width: i32, height: i32) -> Config {
        Config {
            screen_width: width,
            screen_height: height,
        }
    }

    pub fn from_args(a: Vec<String>) -> Config {
        Config {
            screen_width: a[1].parse::<i32>().unwrap(),
            screen_height: a[2].parse::<i32>().unwrap(),
        }
    }
}
