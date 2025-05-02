use esp_hal::{
    clock::Clocks,
    delay::Delay,
    peripherals::{
        TIMG0,
        RADIO_CLK,
        RNG,
        WIFI
    },
    rng::Rng,
    timer::timg::TimerGroup,
};

use esp_wifi::{
    EspWifiInitFor,
    current_millis,
    initialize,
    wifi::{
        AuthMethod,
        ClientConfiguration,
        Configuration,
        WifiController,
        WifiDevice,
        WifiStaDevice,
        utils::create_network_interface,
    },
    wifi_interface::{WifiStack, Socket},
};
use smoltcp::iface::{Interface, SocketSet, SocketStorage};

// Init wifi
pub fn init<'a>(
    timg0: &mut TIMG0,
    rng: &mut RNG,
    radio_clk: RADIO_CLK,
    wifi: &'a mut WIFI,
    clocks: &'a Clocks,
    socket_storage: &'a mut [SocketStorage<'a>; 3],
) -> (
    WifiController<'a>,
    WifiStack<'a, WifiStaDevice>,
) {
    let (
        interface,
        wifi_device,
        wifi_controller,
        socket_set,
    ) = get_wifi_interface(
        timg0,
        rng,
        radio_clk,
        wifi, clocks,
        socket_storage,
    );

    let wifi_stack = get_wifi_stack(
        interface,
        wifi_device,
        socket_set,
    );

    (wifi_controller, wifi_stack)
}

// Get iface needed for wifi
fn get_wifi_interface<'a>(
    timg0: &mut TIMG0,
    rng: &mut RNG,
    radio_clk: RADIO_CLK,
    wifi: &'a mut WIFI,
    clocks: &'a Clocks,
    socket_storage: &'a mut [SocketStorage<'a>; 3],
) -> (
    Interface,
    WifiDevice<'a, WifiStaDevice>,
    WifiController<'a>,
    SocketSet<'a>,
) {
    // Init timer for wifi
    let timg0 = TimerGroup::new(timg0, &clocks);
    let init = initialize(
        EspWifiInitFor::Wifi,
        timg0.timer0,
        Rng::new(rng),
        radio_clk,
        &clocks,
    )
    .unwrap();

    create_network_interface(
        &init,
        wifi,
        WifiStaDevice,
        socket_storage,
    ).unwrap()
}

// Create wifi stack
fn get_wifi_stack<'a>(
    interface: Interface,
    wifi_device: WifiDevice<'a, WifiStaDevice>,
    socket_set: SocketSet<'a>,
) -> WifiStack<'a, WifiStaDevice> {
    WifiStack::new(
        interface,
        wifi_device,
        socket_set,
        current_millis
    )
}

// Connect to wifi
pub fn connect(
    wifi_controller: &mut WifiController,
    delay: &Delay,
) {
    const SSID: &str = env!("SSID");
    const PASSWORD: &str = env!("PASSWORD");

    // Set config
    let client_config = Configuration::Client(
        ClientConfiguration {
            ssid: SSID.try_into().unwrap(),
            password: PASSWORD.try_into().unwrap(),
            auth_method: get_auth_method(),
            ..Default::default()
        }
    );
    wifi_controller
        .set_configuration(&client_config)
        .unwrap();

    // Connect to wifi
    wifi_controller.start().unwrap();
    wifi_controller.connect().unwrap();

    // Blocking wait until it's connected
    while !wifi_controller.is_connected().unwrap() {
        delay.delay_millis(100 as u32);
    }
}

// Check wifi auth method
fn get_auth_method() -> AuthMethod {
    const PASSWORD: &str = env!("PASSWORD");

    let auth_method = match PASSWORD.is_empty() {
        true => AuthMethod::None,
        false => AuthMethod::WPA2Personal,
    };

    auth_method
}

// Get wifi socket
pub fn get_socket<'s, 'n>(
    wifi_stack: &'s mut WifiStack<'n, WifiStaDevice>,
    rx_buffer: &'n mut [u8; 1536],
    tx_buffer: &'n mut [u8; 1536],
) -> Socket<'s, 'n, WifiStaDevice> {
    // Wait for wifi interface to be up
    loop {
        // Process stack tasks
        wifi_stack.work();

        if wifi_stack.is_iface_up() {
            break;
        }
    }

    wifi_stack.get_socket(rx_buffer, tx_buffer)
}
