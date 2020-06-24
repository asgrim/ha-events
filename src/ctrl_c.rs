use crate::ha_webhook;

pub fn set_handler() {
    ctrlc::set_handler(move || {
        ha_webhook::turn_off();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
