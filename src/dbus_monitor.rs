use crate::ha_webhook;
use dbus::arg::{RefArg, Variant};
use dbus::blocking::Connection;
use dbus::message::MatchRule;
use dbus::MessageType;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn watch_for_screen_lock_unlock() {
    let conn = loop {
        match Connection::new_system() {
            Ok(c) => break c,
            Err(e) => {
                warn!("ha-events could not connect to dbus: {}", e);
                thread::sleep(Duration::from_secs(5));
            }
        };
    };

    // Second create a rule to match messages we want to receive; in this example we add no
    // further requirements, so all messages will match
    let mut rule = MatchRule::new();
    rule.msg_type = Some(MessageType::Signal);
    rule.interface = Some("org.freedesktop.DBus.Properties".into());
    rule.path = Some("/org/freedesktop/login1".into());
    rule.eavesdrop = true;

    // Start matching
    conn.add_match(rule, |_: (), _, msg| {
        let mut iter = msg.iter_init();
        if iter.next() {
            let z: HashMap<String, Variant<Box<dyn RefArg>>> = iter.get().unwrap();
            match z.get_key_value("IdleHint") {
                None => (),
                Some(t) => {
                    let tvariant = t.1;
                    let value = &tvariant.0;

                    if let Some(i) = value.as_i64() {
                        if i.eq(&i64::from(1)) {
                            ha_webhook::screen_lock();
                        }
                        if i.eq(&i64::from(0)) {
                            ha_webhook::screen_unlock();
                        }
                    }
                }
            };
        }
        true
    })
    .expect("add_match failed");

    info!("Connected to dbus.");
    loop {
        conn.process(Duration::from_millis(1000)).unwrap();
    }
}
