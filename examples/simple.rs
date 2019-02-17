// A short example displaying the basic usage of the library

fn main() {
    // First the client has to be initialized, this is done at the same time as the connection is established
    let mut client = liboxidizer::connect("liboxidizer_integration", "ws://127.0.0.1:50000").unwrap();

    // Methods can then be called which execute calls to krpc, like get_status
    let result = client.get_status().unwrap();
    let name = client.get_client_name().unwrap();

    println!("client connected: {}", name.name);
    println!("version: {}", result.version);
    println!("bytes read: {}", result.bytes_read);
    println!("bytes written: {}", result.bytes_written);
}
