use std::time::Duration;
use std::thread::sleep;

pub fn wait_for_connectivity() {
    info!("checking if you are online");

    loop {
        if online::online(Some(Duration::from_secs(3))) == Ok(true) {
            info!("you are online!");
            break;
        }

        warn!("No connectivity; checking again in 5s...");
        sleep(Duration::from_secs(5));
    }
}
