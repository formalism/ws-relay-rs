use std::env;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown, SocketAddr};
use std::thread;

fn handler(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf : Vec<u8> = vec![0; 128];
    let mut buf2 : Vec<u8> = vec![0; 640*480];
    loop {
        let res = stream.read(buf.as_mut_slice());
        match res {
            Ok(nbytes) => {
//                println!("{}", nbytes);
                if nbytes == 0 {
                    break Ok(());
                }else{
                    for j in 0..480 {
                        for i in 0..640 {
                            buf2[j*640+i] = (i+j+buf[0] as usize) as u8;
                        }
                    }
                    stream.write(&buf2);
                    stream.flush()?;
                }
            }
            Err(e) => {
                println!("error: {}", e);
                stream.shutdown(Shutdown::Both).unwrap();
                break Ok(());
            }
        }
    }
}

fn main(){
    let args : Vec<String> = env::args().collect();
    let mut port : u16 = 1234;
    for i in 0..args.len() {
        match args[i].parse() {
            Ok(v) => {
                println!("wait on port {}", v);
                port = v;
            }
            Err(_) => {}
        }
    }
    let adrs = [SocketAddr::from(([0, 0, 0, 0], port))];
    let listener = TcpListener::bind(&adrs[..]).expect("failed to bind");
    for streams in listener.incoming(){
        match streams {
            Err(e) => { println!("error {}", e)}
            Ok(stream) => {
                thread::spawn(move || {
                    handler(stream).unwrap_or_else(|error| println!("{:?}", error));
                });
            }
        }
    }
}
