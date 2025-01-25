pub struct Options {
    pub output: String,
    pub input: String,
    pub entry: String,
    pub time: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            output: String::new(),
            input: String::new(),
            entry: "main.main".to_string(),
            time: false,
        }
    }
}
