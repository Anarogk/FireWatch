use embedded_mqtt::{Client, Message, Topic};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct FireAlert {
    pub location: &'static str,
    pub temperature: f32,
    pub smoke_level: u16,
    pub cluster: Vec<&'static str>,
}

impl FireAlert {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn send_fire_alert(mqtt_client: &mut Client, alert: FireAlert) {
    let topic = Topic::new("building/fire-alert").unwrap();
    let message = Message::new(topic, alert.to_json());

    if let Err(e) = mqtt_client.publish(message) {
        log::error!("Failed to send fire alert: {:?}", e);
    } else {
        log::info!("Fire alert sent successfully!");
    }
}
