mod logging;
mod ctrl_c;
mod dbus_monitor;
mod docking_check;
mod ha_webhook;

#[macro_use]
extern crate log;

fn main() {
    logging::setup_logging();

    info!("ha-events starting");

    if !docking_check::is_monitor_attached() {
        warn!("Exiting: did not detect the correct external monitor; not docked correctly");
        return;
    }

    ctrl_c::set_handler();
    ha_webhook::turn_on();
    dbus_monitor::watch_for_screen_lock_unlock();

    // Theoretically, we never get here since ctrl_c handles termination, but just in case...
    ha_webhook::turn_off();
    info!("--FIN--");
}
