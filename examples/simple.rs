fn main() {
    //common::main();

    //just a short period to wait for thread without communication
    //let seconds = time::Duration::from_secs(1);
    //thread::sleep(seconds);
    let mut client = liboxidizer::connect("liboxidizer_integration", "ws://127.0.0.1:50000").unwrap();
    let result = client.get_status().unwrap();
    println!("{}", result.version);
}
