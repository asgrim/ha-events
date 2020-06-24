use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() {
    set_ctrl_c_handler();
    turn_on();
    watch_dbus_for_screen_lock_unlock();
    turn_off();
}

fn set_ctrl_c_handler() {
    ctrlc::set_handler(move || {
        turn_off();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

fn watch_dbus_for_screen_lock_unlock() {
    let process = match Command::new("/usr/bin/dbus-monitor")
        .arg("--session")
        .arg("type='signal',interface='org.gnome.ScreenSaver'")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("could not spawn dbus-monitor: {}", why),
        Ok(process) => process,
    };

    let stdout = process.stdout.unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    for line in stdout_lines {
        let unwrapped_line = line.unwrap();

        if unwrapped_line.contains("boolean true") {
            screen_lock();
            continue;
        }
        if unwrapped_line.contains("boolean false") {
            screen_unlock();
            continue;
        }
    }
}

fn screen_lock() {
    send_webhook_to_ha("ha-pc-lock");
}

fn screen_unlock() {
    send_webhook_to_ha("ha-pc-unlock");
}

fn turn_on() {
    send_webhook_to_ha("ha-pc-on");
}

fn turn_off() {
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
