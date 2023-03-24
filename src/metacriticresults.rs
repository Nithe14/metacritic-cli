pub enum TSP {
    TITLE,
    SCORE,
    PLATFORM,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetacriticResult {
    pub title: String,
    pub score: String,
    pub platform: String,
}
impl MetacriticResult {
    pub fn new(
        title: Option<String>,
        score: Option<String>,
        platform: Option<String>,
    ) -> MetacriticResult {
        MetacriticResult {
            title: title.unwrap_or(String::from("")),
            score: score.unwrap_or(String::from("")),
            platform: platform.unwrap_or(String::from("")),
        }
    }

    pub fn put_data(&mut self, input_data: String, dtype: TSP) {
        match dtype {
            TSP::TITLE => self.title = input_data,
            TSP::SCORE => self.score = input_data,
            TSP::PLATFORM => self.platform = input_data,
        }
    }
}
