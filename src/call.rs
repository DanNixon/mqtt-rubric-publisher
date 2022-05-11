use crate::SendViaDapnet;
use anyhow::Result;
use async_trait::async_trait;
use dapnet_api::{self, Client};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Call {
    recipients: Vec<String>,
    transmitter_groups: Vec<String>,
}

#[async_trait]
impl SendViaDapnet for Call {
    async fn send(&self, client: &Client, text: &str) -> Result<()> {
        let call = dapnet_api::Call::new(
            text.to_string(),
            self.recipients.clone(),
            self.transmitter_groups.clone(),
        );
        log::info!("Generated: {:?}", call);
        client.new_call(&call).await
    }
}
