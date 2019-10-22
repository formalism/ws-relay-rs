extern crate websocket;

use std::env;
use std::thread;
use std::net::{ToSocketAddrs, SocketAddr, Shutdown};
use std::net::TcpStream;
use std::io::{Read, Write};

use websocket::Message;
use websocket::sync::Server;
use websocket::OwnedMessage::{Text, Binary, Close, Ping, Pong};

fn main() {
    let mut serv = "127.0.0.1";
    let mut port = 1234;
    let mut waitport = 3333;
    let args : Vec<String> = env::args().collect();
    let mut i = 0;
    loop {
        if i >= args.len() {
            break;
        }
        if args[i] == "-p" || args[i] == "-P" {
            port = args[i+1].parse().unwrap();
            i = i + 1;
        }
        if args[i] == "-s" || args[i] == "-S" {
            serv = &args[i+1];
            i = i + 1;
        }
        if args[i] == "-w" || args[i] == "-W" {
            waitport = args[i+1].parse().unwrap();
            i = i + 1;
        }
        i = i + 1;
    }
    println!("connect to {}:{}", serv, port);
    let host_and_port = format!("{}:{}", serv, port);
    let mut srv_adrs = host_and_port.to_socket_addrs().unwrap();
    let adr = SocketAddr::from(([127, 0, 0, 1], waitport));
    let server = Server::bind(adr.to_socket_addrs().unwrap().next().unwrap()).unwrap();

    let addr = srv_adrs.find(|x| (*x).is_ipv4()).unwrap();

    for connection in server.filter_map(Result::ok){
        let client = connection.accept().unwrap();
        let (mut receiver, mut sender) = client.split().unwrap();
        println!("connect to TCP server");
        let mut stream = TcpStream::connect(addr).unwrap();
        let mut st = stream.try_clone().unwrap();
        let mut st2 = stream.try_clone().unwrap();

        thread::spawn(move || { // WebSocket -> TCP
            loop {
                let res = receiver.recv_message().unwrap();
                //println!("message from WS");
                match res{
                    Text(tx) => {
                        st.write(tx.as_bytes());
                    }
                    Binary(v) => {
                        //println!("bin len={}", v.len());
                        let r = stream.write(&v);
                        match r {
                            Err(e) => { println!("write error: {}", e); }
                            Ok(_) => {}
                        }
                    }
                    Close(_) => { 
                        println!("WebSocket Closed");
                        st.shutdown(Shutdown::Both);
                        break;
                    }
                    Ping(_) => {}
                    Pong(_) => {}
                }
            }
        });
        thread::spawn(move || { // TCP -> WebSocket
//            st2.set_read_timeout(Some(Duration::new(1, 0))).expect("failed set_read_timeout");
            loop {
                let mut buf = vec![0; 65536];
                let ret = st2.read(buf.as_mut_slice());
                //println!("read from TCP");
                match ret {
                    Err(_) => {
                        println!("TCP connection to server closed");
                        break;
                    }
                    Ok(len) => {
                        if len != 0 {
                            buf.resize(len, 0);
                            let msg = Message::binary(buf);
                            sender.send_message(&msg);
                        }else{
                            println!("read 0 bytes");
                            break;
                        }
                    }
                }
            }
        });
    }
}
