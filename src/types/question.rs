use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}