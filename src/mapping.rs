use crate::{call::Call, rubric::Rubric, SendViaDapnet};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Mapping {
    topic: String,
    pub destination: Destination,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum Destination {
    Call(Call),
    Rubric(Rubric),
}

#[async_trait]
impl SendViaDapnet for Destination {
    async fn send(&self, client: &dapnet_api::Client, text: &str) -> Result<()> {
        match &self {
            Destination::Call(i) => i.send(client, text).await,
            Destination::Rubric(i) => i.send(client, text).await,
        }
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

    pub(crate) fn get_topics(&self) -> Vec<String> {
        self.mapping.iter().map(|m| m.topic.clone()).collect()
    }

    pub(crate) fn lookup_by_topic(&self, topic: &str) -> Option<&Mapping> {
        self.mapping.iter().find(|m| m.topic == topic)
    }
}
