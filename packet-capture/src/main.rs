use std::env;

use pnet::datalink::Channel;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please specify target interface name");
        std::process::exit(1);
    }

    let interface_name = &args[1];
    let interfaces = pnet::datalink::interfaces();
    let intarface = interfaces
        .into_iter()
        .filter(|iface| iface.name == *interface_name)
        .next()
        .expect("Failed to get interface");

    let (_tx, mut rx) = match pnet::datalink::channel(&intarface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => {
            panic!("Failed to create datalink channel {}", e)
        }
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                // TODO
            }
            Err(e) => {
                panic!("Failed to read: {}", e);
            }
        }
    }
}
