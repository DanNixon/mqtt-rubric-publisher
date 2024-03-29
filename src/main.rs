mod call;
mod mapping;
mod rubric;

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use dapnet_api::Client;
use mapping::Mappings;
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, PersistenceType};
use tokio::time::Duration;

#[async_trait]
trait SendViaDapnet {
    async fn send(&self, client: &Client, text: &str) -> Result<()>;
}

/// Tool to publish calls and news to DAPNET via MQTT.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Address of MQTT broker to connect to
    #[clap(long, env = "MQTT_BROKER", default_value = "tcp://localhost:1883")]
    mqtt_broker: String,

    /// Client ID to use when connecting to MQTT broker
    #[clap(long, env = "MQTT_CLIENT_ID", default_value = "mqtt-rubric-publisher")]
    mqtt_client_id: String,

    /// MQTT QoS, must be 0, 1 or 2
    #[clap(long, env = "MQTT_QOS", default_value = "0")]
    mqtt_qos: i32,

    /// MQTT username
    #[clap(long, env = "MQTT_USERNAME", default_value = "")]
    mqtt_username: String,

    /// MQTT password
    #[clap(long, env = "MQTT_PASSWORD", default_value = "")]
    mqtt_password: String,

    /// DAPNET username
    #[clap(long, env = "DAPNET_USERNAME")]
    dapnet_username: String,

    /// DAPNET password
    #[clap(long, env = "DAPNET_PASSWORD")]
    dapnet_password: String,

    /// Path to topic-rubric mapping file
    #[clap(long, env = "MAPPING_FILE", default_value = "./mapping.toml")]
    mapping_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Cli::parse();

    let mapping = Mappings::from_file(&args.mapping_file)?;
    log::info!("Loaded mapping: {:#?}", mapping);

    let dapnet_client = Client::new(&args.dapnet_username, &args.dapnet_password);

    let mut mqtt_client = AsyncClient::new(
        CreateOptionsBuilder::new()
            .server_uri(&args.mqtt_broker)
            .client_id(&args.mqtt_client_id)
            .persistence(PersistenceType::None)
            .finalize(),
    )?;

    let stream = mqtt_client.get_stream(25);

    let topics = mapping.get_topics();
    mqtt_client.set_connected_callback(move |c| {
        log::info!("Connected to broker");
        for topic in &topics {
            c.subscribe(topic, args.mqtt_qos);
        }
    });

    let response = mqtt_client
        .connect(
            ConnectOptionsBuilder::new()
                .clean_session(true)
                .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(5))
                .keep_alive_interval(Duration::from_secs(5))
                .user_name(&args.mqtt_username)
                .password(&args.mqtt_password)
                .finalize(),
        )
        .wait()?;

    log::info!(
        "Using MQTT version {}",
        response.connect_response().unwrap().mqtt_version
    );

    loop {
        if let Ok(Some(msg)) = stream.recv().await {
            if let Some(mapping) = mapping.lookup_by_topic(msg.topic()) {
                log::info!("Received MQTT message matching: {:?}", mapping);
                if let Err(e) = mapping
                    .destination
                    .send(&dapnet_client, &msg.payload_str())
                    .await
                {
                    log::error!("Failed to send with error {}", e);
                }
            }
        }
    }
}
