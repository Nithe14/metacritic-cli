pub enum TSPD {
    TITLE,
    SCORE,
    PLATFORM,
    DATE,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetacriticResult {
    pub title: String,
    pub score: String,
    pub platform: String,
    pub release_date: String,
}
impl MetacriticResult {
    pub fn new() -> MetacriticResult {
        MetacriticResult {
            title: String::new(),
            score: String::new(),
            platform: String::new(),
            release_date: String::new(),
        }
    }

    pub fn put_data(&mut self, input_data: String, dtype: TSPD) {
        match dtype {
            TSPD::TITLE => self.title = input_data,
            TSPD::SCORE => self.score = input_data,
            TSPD::PLATFORM => self.platform = input_data,
            TSPD::DATE => self.release_date = input_data,
        }
    }
}
