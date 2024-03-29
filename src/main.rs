use std::io;
use std::net::{TcpStream};
use colored::Colorize;

fn main() {
    println!("Welcome to the port scanner! (made by not you)");
    println!("Enter the IP address to check:");
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).expect("Failed to read IP address");
    let ip = ip.trim();

    println!("Enter the ports to check, separated by commas (e.g. 80,443,8080):");
    let mut ports = String::new();
    io::stdin().read_line(&mut ports).expect("Failed to read ports");
    println!("If not responding in about 10 seconds the port is propably closed, you don't have to wait for it to finish ;)");
    let ports: Vec<_> = ports.trim().split(',').map(|p| p.trim().parse::<u16>().unwrap()).collect();

    for port in ports {
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(_) => println!("{} {} {}", "Port".green(), port, "is open".green()),
            Err(_) => println!("{} {} {}", "Port".green(), port, "is closed".red()),
        }
    }

    println!("Want do check another IP? Click ENTER if no type N");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read answer");
    if answer.trim() == "y" || answer.trim() == "Y" || answer.trim().is_empty() {
        main();
    } else {
        std::process::exit(0);
        
    }
}
