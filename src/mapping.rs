use anyhow::Result;
use dapnet_api::News;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Mapping {
    topic: String,

    #[serde(rename = "rubric")]
    rubric_name: String,

    #[serde(rename = "number")]
    rubric_number: i8,
}

impl Mapping {
    pub(crate) fn create_news(&self, text: &str) -> News {
        let mut news = News::new(self.rubric_name.clone(), text.to_string());
        news.number = Some(self.rubric_number);
        news
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Mappings {
    mapping: Vec<Mapping>,
}

impl Mappings {
    pub(crate) fn from_file(filename: &str) -> Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(filename)?)?)
    }

    pub(crate) fn get_topics(&self) -> Vec<&str> {
        self.mapping.iter().map(|m| m.topic.as_str()).collect()
    }

    pub(crate) fn lookup_by_topic(&self, topic: &str) -> Option<&Mapping> {
        self.mapping.iter().find(|m| m.topic == topic)
    }
}
