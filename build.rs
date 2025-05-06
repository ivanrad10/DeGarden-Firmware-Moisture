fn main() {
    if let Ok(iter) = dotenvy::dotenv_iter() {
        for item in iter.flatten() {
            println!("cargo:rustc-env={}={}", item.0, item.1);
        }
    }

    if let Ok(ssid) = std::env::var("SSID") {
        println!("cargo:rustc-env=SSID={}", ssid);
    }

    if let Ok(password) = std::env::var("PASSWORD") {
        println!("cargo:rustc-env=PASSWORD={}", password);
    }

    if let Ok(device_id) = std::env::var("DEVICE_ID") {
        println!("cargo:rustc-env=DEVICE_ID={}", device_id);
    }
}
