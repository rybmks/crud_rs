use std::thread;

use back::*;

#[tokio::main]
async fn main() {
    let t = thread::spawn(|| web_server::start_web_server().unwrap());

    t.join().unwrap();
}
