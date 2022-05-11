use crate::SendViaDapnet;
use anyhow::Result;
use async_trait::async_trait;
use dapnet_api::{Client, News};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Rubric {
    name: String,
    number: i8,
}

#[async_trait]
impl SendViaDapnet for Rubric {
    async fn send(&self, client: &Client, text: &str) -> Result<()> {
        let mut news = News::new(self.name.clone(), text.to_string());
        news.number = Some(self.number);
        log::info!("Generated: {:?}", news);
        client.new_news(&news).await
    }
}
