use esp_hal::{
    clock::Clocks,
    delay::Delay,
    peripherals::{RADIO_CLK, RNG, TIMG0, WIFI},
};

use smoltcp::iface::SocketStorage;

use esp_println::println;

mod network;
mod wifi;

// Send data
pub fn send(
    timg0: &mut TIMG0,
    rng: &mut RNG,
    radio_clk: RADIO_CLK,
    wifi: &mut WIFI,
    clocks: &Clocks,
    delay: &Delay,
    msg: &str,
) {
    // Create socket storage
    let mut socket_storage: [SocketStorage; 3] = Default::default();

    // Init wifi
    let (mut wifi_controller, mut wifi_stack) =
        wifi::init(timg0, rng, radio_clk, wifi, clocks, &mut socket_storage);

    // Connect to wifi
    wifi::connect(&mut wifi_controller, delay);

    println!("Connected to Wifi");

    // Create buffers and socket
    let mut rx_buffer = [0u8; 1536];
    let mut tx_buffer = [0u8; 1536];
    let mut socket = wifi::get_socket(&mut wifi_stack, &mut rx_buffer, &mut tx_buffer);

    // Send request
    println!("Sending Data");

    network::post_req("/moisture", &mut socket, msg);

    delay.delay_millis(5000 as u32);
}
