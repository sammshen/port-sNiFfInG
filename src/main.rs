// cargo run -- arguments
// no arguments: automatically runs on local host (w/ default 10 threads)
// -h or -help: input scheme information
// -i ip_addr: runs on specified address (w/ default 10 threads)
// -t num_threads: runs with specified number of threads
// -p start_port-end_port

use std::{env, process, net::IpAddr};

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
    // println!("{:?}", args)
    
}
















// use std::env;
// use std::io::{self, Write};
// use std::net::{IpAddr, TcpStream};
// use std::str::FromStr;
// use std::process;
// use std::sync::mpsc::{Sender, channel};
// use std::thread;

// const MAX: u16 = 65535;
// struct Arguments {
//     flag: String,
//     ipaddr: IpAddr,
//     threads: u16,
// }

// impl Arguments {
//     fn new(args: &[String]) -> Result<Arguments, &'static str> {
//         if args.len() < 2 {
//             return Err("not enough arguments");
//         } else if args.len() > 4 {
//             return Err("too many arguments");
//         }
//         let f = args[1].clone();
//         if let Ok(ipaddr) = IpAddr::from_str(&f) {
//             return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});
//         } else {
//             let flag = args[1].clone();
//             if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
//                 println!("Usage: -j to select how many threads you want
//                 \r\n      -h or -help to show this help message");
//                 return Err("help");
//             } else if flag.contains("-h") || flag.contains("-help") {
//                 return Err("too many arguments");
//             } else if flag.contains("-j") {
//                 let ipaddr = match IpAddr::from_str(&args[3]) {
//                     Ok(s) => s,
//                     Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6")
//                 };
//                 let threads = match args[2].parse::<u16>() {
//                     Ok(s) => s,
//                     Err(_) => return Err("failed to parse thread number")
//                 };
//                 return Ok(Arguments {threads, flag, ipaddr});
//             } else {
//                 return Err("invalid syntax");
//             }
//         }
//     }
// }

// fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
//     let mut port: u16 = start_port + 1;
//     loop {
//         match TcpStream::connect((addr, port)) {
//             Ok(_) => {
//                 print!(".");
//                 io::stdout().flush().unwrap();
//                 tx.send(port).unwrap();
//             },
//             Err(_) => {}
//         }

//         if (MAX - port) <= num_threads {
//             break;
//         }
//         port += num_threads;
//     }
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let program = args[0].clone();
//     let arguments = Arguments::new(&args).unwrap_or_else(
//         |err| {
//             if err.contains("help") {
//                 process::exit(0);
//             } else {
//                 eprintln!("{} problem parsing arguments: {}", program, err);
//                 process::exit(0);
//             }
//         }
//     );

//     let num_threads = arguments.threads;
//     let (tx, rx) = channel();
//     for i in 0..num_threads {
//         let tx = tx.clone();
        
//         thread::spawn(move || {
//             scan(tx, i, arguments.ipaddr, num_threads);
//         });
//     }

//     let mut out = vec![];
//     drop(tx);
//     for p in rx {
//         out.push(p);
//     }

//     println!("");
//     out.sort();
//     for v in out {
//         println!("{} is open", v);
//     }
// }