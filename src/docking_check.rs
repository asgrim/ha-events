use edid::Descriptor;
use std::fs::File;
use std::io::Read;
use std::env;

pub fn is_monitor_attached() -> bool {
    // Use `find /sys -name edid` to find this...
    let edid_file_path = match env::var("EDID_FILE_PATH") {
        Ok(t) => t,
        Err(_) => String::from("")
    };
    // Check the EDID - should be the `ProductName` - uncomment the println to dump the EDID values
    let expected_monitor_name = match env::var("EXPECTED_MONITOR_ATTACHED") {
        Ok(t) => t,
        Err(_) => String::from("")
    };

    // If the environment variables are not set, assume we're not checking for monitor-attached
    if edid_file_path.eq("") || expected_monitor_name.eq("") {
        info!("EDID_FILE_PATH or EXPECTED_MONITOR_ATTACHED environment variables not set: assuming we are docked");
        return true;
    }

    let edid_bytes = read_file_as_byte_vec(edid_file_path.as_str());
    if edid_bytes.len() == 0 {
        warn!("EDID_FILE_PATH did not exist or was an empty file");
        return false;
    }
    let edid_descriptors = edid::parse(edid_bytes.as_slice()).unwrap().1.descriptors;
    // Unsure about what ProductName is? Uncomment this, look for the `ProductName` enumerator...
    // println!("{:#?}", edid_descriptors);
    if get_display_name_from_edid_vector(edid_descriptors).ne(expected_monitor_name.as_str()) {
        warn!("Could not extract ProductName enumerator from EDID file: {}", edid_file_path);
        return false;
    }
    true
}

fn get_display_name_from_edid_vector(options: Vec<Descriptor>) -> String {
    for opt in options {
        if let Descriptor::ProductName(ref data) = opt {
            return data.clone();
        }
    }
    String::from("")
}

fn read_file_as_byte_vec(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("File open failed");

    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read");

    return data;
}
