use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{Sender};

// max 65535
pub fn scan(tx: Sender<u16>, start: u16, ip: IpAddr, threads: u16) {
    let mut port: u16 = start + 1;
    loop {
        match TcpStream::connect((ip, port)) {
            Ok(_) => {
                print!("✓");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {}
        };

        if (65535 - port) <= threads {
            break;
        }

        port += threads;
    }
}