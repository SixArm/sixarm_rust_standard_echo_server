use std::net::{IpAddr, Ipv4Addr, TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;
use std::thread;

macro_rules! host {
    () => ( IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)) )
}

macro_rules! port {
    () => ( 8888 )
}

macro_rules! bind {
    () => ( format!("{}:{}", host!(), port!()) )
}

macro_rules! bind_err_msg {
    () => ( format!("Expect bind:{}", bind!()) )
}

macro_rules! local_addr_err_msg {
    () => ( format!("Expect local addr:{}", bind!()) )
}

macro_rules! connection_err_msg {
    () => ( format!("Expect connection:{}", bind!()) )
}

fn main() {
    let listener = TcpListener::bind(bind!())
        .expect(&bind_err_msg!());
    let _addr = listener.local_addr()
        .expect(&local_addr_err_msg!());
    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
            panic!("{}:{}", connection_err_msg!(), e);
        }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 16];
    if let Err(_) = stream.write("HELLO\n".as_bytes()) {
        return;
    }
    while let Ok(read) = stream.read(&mut buffer) {
        if read == 0 {
        break;
        }
        if let Err(_) = stream.write(&buffer[0..read]) {
            break;
        }
    }
}
