use heapless::String;
use embedded_io::{Write, Read};

use esp_wifi::{
    wifi::WifiStaDevice,
    wifi_interface::Socket,
};
use smoltcp::wire::{IpAddress, Ipv4Address};

// Dev only
use esp_println::println;

// Execute post request
pub fn post_req(
    endpoint: &str,
    socket: &mut Socket<WifiStaDevice>,
    msg: &str,
) {
    let ip_address = get_ip();
    let request = form_post_req(endpoint, msg);
    send_req(socket, ip_address, request);
    read_res(socket);
}

// Make server IP
fn get_ip() -> IpAddress{
    const IP_OCTET: &str = env!("IP_OCTET");

    let bytes = IP_OCTET.as_bytes();
    let mut parts = [0u8; 4];
    let mut part_index = 0;
    let mut current = 0u16;

    for &b in bytes {
        if b == b'.' {
            parts[part_index] = current as u8;
            current = 0;
            part_index += 1;
        } else if b >= b'0' && b <= b'9' {
            current = current * 10 + (b - b'0') as u16;
        }
    }
    parts[part_index] = current as u8;

    IpAddress::Ipv4(
        Ipv4Address::new(parts[0], parts[1], parts[2], parts[3])
    )
}

// Form post request string
fn form_post_req(endpoint: &str, body: &str) -> String<256> {
    const HOST: &str = env!("HOST");

    let mut request: String<256> = String::new();
    request.push_str("POST ").unwrap();
    request.push_str(endpoint).unwrap();
    request.push_str(" HTTP/1.1\r\n").unwrap();
    request.push_str("Host: ").unwrap();
    request.push_str(HOST).unwrap();
    request.push_str("\r\n").unwrap();
    request.push_str("Content-Type: application/json\r\n").unwrap();
    request.push_str("Content-Length: ").unwrap();
    request
        .push_str(itoa::Buffer::new().format(body.len()))
        .unwrap();
    request.push_str("\r\nConnection: close\r\n\r\n").unwrap();
    request.push_str(body).unwrap();

    request
}

// Send HTTP request to server
fn send_req(
    socket: &mut Socket<WifiStaDevice>,
    ip_address: IpAddress,
    request: String<256>,
) {
    const PORT: &str = env!("PORT");
    let port: u16 = PORT.parse::<u16>().unwrap();

    socket.work();
    socket.open(ip_address, port).unwrap();
    socket.write(request.as_bytes()).unwrap();
}

// Read HTTP response from server
fn read_res(
    socket: &mut Socket<WifiStaDevice>,
) {
    loop {
        let mut buffer = [0u8; 512];
        if let Ok(len) = socket.read(&mut buffer) {
            let to_print = core::str::from_utf8(&buffer[..len]).unwrap();
            println!("{}", to_print);

        } else {
            break;
        }

        // todo interrupt for timeout
    }

    socket.disconnect();
}
