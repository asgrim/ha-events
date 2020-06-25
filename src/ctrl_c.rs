use crate::ha_webhook;

pub fn set_handler() {
    ctrlc::set_handler(move || {
        info!("Received Ctrl+C or SIGINT, sending hook");
        ha_webhook::turn_off();
        info!("Exiting.");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
