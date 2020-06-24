use crate::ha_webhook;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn watch_for_screen_lock_unlock() {
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
            ha_webhook::screen_lock();
            continue;
        }
        if unwrapped_line.contains("boolean false") {
            ha_webhook::screen_unlock();
            continue;
        }
    }
}
