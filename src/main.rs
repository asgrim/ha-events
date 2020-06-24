use std::env;

fn main() {
    turn_on();

    ctrlc::set_handler(move || {
        turn_off();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {}
}

fn send_hook(endpoint: &str) {
    let client = reqwest::blocking::Client::new();

    // example - HA_WEBHOOK_BASE_URL=http://your-local-ha-instance/api/webhook/
    // Note: ending slash IS needed in URL
    let url =
        env::var("HA_WEBHOOK_BASE_URL").expect("Set the HA_WEBHOOK_BASE_URL environment variable");

    let str = format!("{}{}", url, endpoint);

    match client.post(&str).send() {
        Ok(_) => {
            println!("Send hook to {}", endpoint);
        }
        Err(e) => {
            println!("Error sending hook to endpoint {}: {:?}", endpoint, e);
        }
    }
}

fn turn_on() {
    send_hook("pc-on");
}

fn turn_off() {
    send_hook("pc-off");
}
