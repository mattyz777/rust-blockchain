pub struct Profile {
    pub first_name: String,
    pub last_name: String,
}


impl Profile {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

}