use std::io;
use std::net::{TcpStream};


fn main() {
    println!("Welcome to the port scanner! (made by not you)");
    println!("Enter the IP address to check:");
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Failed to read IP address");
    let ip = ip.trim();

    println!("Enter the ports to check, separated by commas (e.g. 80,443,8080):");
    let mut ports = String::new();
    io::stdin().read_line(&mut ports).expect("Failed to read ports");
    println!("If not responding in 5 seconds the port is propably closed, you don't have to wait for it to finish ;)");
    let ports: Vec<_> = ports.trim().split(',').map(|p| p.trim().parse::<u16>().unwrap()).collect();

    for port in ports {
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(_) => println!("Port {} is open", port),
            Err(_) => println!("Port {} is closed", port),
        }
    }

    println!("Want do check another IP? (y/n)");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read answer");
    if answer.trim() == "y" {
        main();
    } else {
        std::process::exit(0);
        
    }
}
