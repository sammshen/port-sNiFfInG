// cargo run -- arguments
// no arguments: automatically runs on local host (w/ default 10 threads)
// -h or -help: input scheme information
// -i ip_addr: runs on specified address (w/ default 10 threads)
// -t num_threads: runs with specified number of threads
// -p start_port-end_port

use std::{env, io::{self, Write}, process, thread};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;

const MAX_PORT: u16 = 65355;
const DEFAULT_THREADS: u16 = 10;
const MAX_ARGS: usize = 6;

#[derive(Debug)]
struct Arguments {
    addr: IpAddr,
    num_threads: u16,
    start_port: u16,
    end_port: u16
}

impl Arguments {
    fn new(user_input: &[String]) -> Result<Arguments, &'static str> {
        // default values
        let mut addr = "127.0.0.1".parse::<IpAddr>().unwrap();
        let mut num_threads = DEFAULT_THREADS;
        let mut start_port = 1;
        let mut end_port = MAX_PORT;

        if user_input.len() > MAX_ARGS {
            Err("too many arguments")
        } else if user_input.contains(&"-h".to_string())
               || user_input.contains(&"-help".to_string()) {
            if user_input.len() == 1 {
                println!(
                "cargo run -- argument scheme:
                no arguments: default values (see below)
                -h or -help: input scheme information
                ip_addr: runs on specified address (default is local host)
                -t num_threads: runs with specified number of threads (default is 10 threads)
                -p start_port-end_port (default is all {MAX_PORT} ports)");
                Err("help")
            } else {
                Err("use -h or -help as an isolated argument")
            }
        } else {
            let mut iter = user_input.iter();
            while let Some(arg) = iter.next() {
                match arg.as_str() {
                    "-i" => {
                        let potential_address = iter.next()
                                                         .ok_or("No IP address provided after -i")?;
                        addr = potential_address.parse::<IpAddr>()
                               .map_err(|_| "Invalid IP Address")?;
                    }
                    "-t" => {
                        let potential_thread_nums = iter.next()
                                                    .ok_or("No thread numbers provided after -t")?;
                        num_threads = potential_thread_nums.parse::<u16>()
                                      .map_err(|_| "Invalid thread numbers")?;
                        if num_threads == 0 || num_threads > MAX_PORT {
                            return Err("Thread number out of range");
                        }
                    }
                    "-p" => {
                        let potential_ports = iter.next()
                                              .ok_or("No start and ending port provided after -p")?;
                        let ports: Vec<&str> = potential_ports.split("-").collect();
                        if ports.len() != 2 {
                            return Err("Ports should be in the form port1-port2");
                        }
                        let port1 = ports[0].parse::<u16>().map_err(|_| "Invalid first port")?;
                        let port2 = ports[1].parse::<u16>().map_err(|_| "Invalid second port")?;
                        if port1 > port2 {
                            return Err("Starting port needs to be less than Ending port");
                        } if port2 > MAX_PORT {
                            return Err("Ports must be in range 0-{MAX_PORT}");
                        }
                        start_port = port1;
                        end_port = port2;
                    }
                    _ => {return Err("invalid argument")}
                }
            }
            Ok(Arguments {addr, num_threads, start_port, end_port})
        }
    }
}

fn scan(thread_num: u16, tx: Sender<u16>, args: Arc<Arguments>){
    let mut port: u16 = (*args).start_port + thread_num;
    while port < (*args).end_port {
        if let Ok(_) = TcpStream::connect(((*args).addr, port)) {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        port += (*args).num_threads;
    }
}
fn main() {
    let user_input: Vec<String> = env::args().skip(1).collect();
    let args: Arguments = Arguments::new(&user_input).unwrap_or_else(
        |err| {
            if err != "help" {
                eprintln!("Error: {}", err);
            }
            process::exit(1);
        }
    );
    println!("Arguments: {:?}", args);
    let wrapped_args = Arc::new(args);

    let (tx, rx) = channel();
    for thread_num in 0..(*wrapped_args).num_threads {
        let tx = tx.clone();
        let args_clone = Arc::clone(&wrapped_args);
        thread::spawn(move || {
            scan(thread_num, tx, args_clone);
        });
    }
    drop(tx); // drop sender to close receiver

    // collect open ports in a vector to sort
    let mut out = vec![];
    for open_port in rx {
        out.push(open_port);
    }
    out.sort();
    println!("");
    for open_port in out {
        println!("{open_port} is open!");
    }
}
