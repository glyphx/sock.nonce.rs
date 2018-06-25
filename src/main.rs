extern crate rand;
use rand::{thread_rng, Rng};

use std::io::prelude::*;
use std::env;
use std::net::{TcpListener, TcpStream, Shutdown};

fn main() {
    let opts: Vec<_> = env::args().collect();
    let default_server = "localhost:12125";
    let default_host = "localhost";
    let default_port: i16 = 12126; 
    let mut gpus: i16 = 1;

    if opts.len() <= 1 {
        println!("No argument specified defaulting to 1 GPU");
    } else {
        gpus = opts[1].parse().unwrap_or(1);
    }

    println!("Starting server for {} GPUs", gpus);
    let mut ports: Vec<i16> = Vec::new();
    let mut hosts: Vec<String> = Vec::new();

    for port in default_port..(default_port + gpus) {
        let host = default_host.to_string() + &port.to_string();
        hosts.push(host);
        ports.push(port);
        println!("Will send data via TCP to port {}", port);
    }

    println!("Starting up TCP Server on {}", default_server);
    let server = TcpListener::bind(default_server.to_string()).unwrap();

    for mut stream in server.incoming() {

        match server.accept() {
            Ok((_socket, addr)) => println!("New client: {:?}", addr),
            Err(e) => println!("Couldn't get client: {:?}", e),
        }

        match stream {
            Err(e) => { println!("Malformed request {}", e) }
            Ok(mut stream) => {
                let mut buf = Vec::new();
                stream.read_to_end(&mut buf).unwrap();
                println!("Bytes received: {:?}", buf);
                
                if buf[0] != 1 {
                    for i in 37..45 {
                        let random_number: u8 = thread_rng().gen_range(1, 255);
                        buf[i] = random_number;
                    }
                }

                for host in hosts.clone() {
                    let mut stream = TcpStream::connect(host.to_string()).unwrap();
                    stream.write(&buf).unwrap();
                    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
                    println!("Sent bytes {:?} to {}", buf, host);
                }
            }
        }

    }
}
