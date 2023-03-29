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
    pub date: String,
}
impl MetacriticResult {
    pub fn new(
        title: Option<String>,
        score: Option<String>,
        platform: Option<String>,
        date: Option<String>,
    ) -> MetacriticResult {
        MetacriticResult {
            title: title.unwrap_or(String::from("")),
            score: score.unwrap_or(String::from("")),
            platform: platform.unwrap_or(String::from("")),
            date: date.unwrap_or(String::from("")),
        }
    }

    pub fn put_data(&mut self, input_data: String, dtype: TSPD) {
        match dtype {
            TSPD::TITLE => self.title = input_data,
            TSPD::SCORE => self.score = input_data,
            TSPD::PLATFORM => self.platform = input_data,
            TSPD::DATE => self.date = input_data,
        }
    }
}
