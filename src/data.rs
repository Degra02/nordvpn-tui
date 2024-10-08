use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub cities: Vec<String>,
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Country {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cities: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_cities(&mut self, cities: &[&str]) {
        for c in cities {
            self.cities.push(c.to_string());
        }
    }
}
