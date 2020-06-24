mod ctrl_c;
mod dbus_monitor;
mod docking_check;
mod ha_webhook;

fn main() {
    if !docking_check::is_monitor_attached() {
        println!("Exiting: did not detect the correct external monitor; not docked correctly");
        return;
    }

    ctrl_c::set_handler();
    ha_webhook::turn_on();
    dbus_monitor::watch_for_screen_lock_unlock();
    ha_webhook::turn_off();
}
