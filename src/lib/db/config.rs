// for use with figment
#[derive(Debug,Clone)]
pub struct Config {
    pub path: String,
    pub temporary: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: "turlsdata".to_owned(),
            temporary: false,
        }
    }
}
