extern crate websocket;

use std::thread;
use websocket::sync::Server;

pub fn main() {
    thread::spawn(move || {
	      let server = Server::bind("127.0.0.1:50000").unwrap();

	      for request in server.filter_map(Result::ok) {
		        thread::spawn(move || {
			          let _client = request.accept().unwrap();
		        });
        }});
	}
