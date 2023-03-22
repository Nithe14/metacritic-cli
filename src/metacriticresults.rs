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
        ititle: Option<String>,
        iscore: Option<String>,
        iplatform: Option<String>,
    ) -> MetacriticResult {
        MetacriticResult {
            title: ititle.unwrap_or(String::from("")),
            score: iscore.unwrap_or(String::from("")),
            platform: iplatform.unwrap_or(String::from("")),
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
