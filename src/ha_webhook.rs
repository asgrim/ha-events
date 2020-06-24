use std::env;

pub fn screen_lock() {
    send_webhook_to_ha("ha-pc-lock");
}

pub fn screen_unlock() {
    send_webhook_to_ha("ha-pc-unlock");
}

pub fn turn_on() {
    send_webhook_to_ha("ha-pc-on");
}

pub fn turn_off() {
    send_webhook_to_ha("ha-pc-off");
}

fn send_webhook_to_ha(webhook_id: &str) {
    let client = reqwest::blocking::Client::new();

    // example - HA_WEBHOOK_BASE_URL=http://your-local-ha-instance/api/webhook/
    // Note: ending slash IS needed in URL
    let ha_webhook_base_url =
        env::var("HA_WEBHOOK_BASE_URL").expect("Set the HA_WEBHOOK_BASE_URL environment variable");

    let full_url = format!("{}{}", ha_webhook_base_url, webhook_id);

    match client.post(&full_url).send() {
        Ok(_) => {
            println!("Send hook to {}", full_url);
        }
        Err(e) => {
            println!("Error sending hook to endpoint {}: {:?}", full_url, e);
        }
    }
}
