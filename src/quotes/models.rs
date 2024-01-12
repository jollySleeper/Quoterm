use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Quote {
    content: String,
    author: String,
}

impl Quote {
    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_author(&self) -> &String {
        &self.author
    }

    pub fn new(content: String, author: String) -> Self {
        Self { content, author }
    }
}
