use std::net::IpAddr;
use std::str::FromStr;

pub struct Args {
    pub flag: String,
    pub ip: IpAddr,
    pub threads: u16
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            panic!("Not enough arguments have been provided!");
        } else if args.len() > 4 {
            panic!("Too many arguments have been provided!");
        }

        if let Ok(ip) = IpAddr::from_str(&args[1].clone()) {
            return Ok(Args { flag: String::from("") , ip, threads: 4});
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!("Usage: --threads, -t; set the amount of threatds to use
                \r\n      --help, -h; to show this message");
                return Err("help");
            } else if flag.contains("-t") || flag.contains("--threads") {
                let ip = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP, must be IPv4 or IPv6")
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread #")
                };

                return Ok(Args { threads, flag, ip });
            } else {
                panic!("Too many arguments or invalid syntax");
            }
        }
    }
}