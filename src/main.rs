use std::env;
use std::io::{self, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", args[0]);
        exit(1);
    }
    let port = &args[1];
    let address = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&address).unwrap_or_else(|err| {
        eprintln!("Failed to open port on {}: {}", port, err);
        exit(1);
    });

    let (sender, receiver) = channel();

    ctrlc::set_handler(move || {
        sender.send(()).unwrap();
    })
    .expect("Error setting Ctrl-C handler");

    thread::spawn(move || {
        receiver.recv().unwrap();
        eprintln!("\nReceived SIGINT or SIGTERM, terminating...");
        exit(0);
    });

    eprintln!("Listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(conn) => {
                handle_connection(conn);
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
                exit(1);
            }
        }
    }
}

fn handle_connection(conn: TcpStream) {
    let mut reader = BufReader::new(conn);
    let mut writer = BufWriter::new(io::stdout());

    if let Err(err) = io::copy(&mut reader, &mut writer) {
        eprintln!("Error reading from connection: {}", err);
    }
}
