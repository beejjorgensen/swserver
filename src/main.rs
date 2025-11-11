use std::env;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::sync::Arc;
use std::{thread, time};

fn consume_input(stream: &mut TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];

    loop {
        match stream.read(&mut buf) {
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
            Ok(0) => return Err(std::io::Error::other("client closed connection")),
            Err(e) => return Err(e),
            Ok(_) => {} // keep consuming
        }
    }

    Ok(())
}

fn show_frame<T: Read>(reader: &mut BufReader<T>, stream: &mut TcpStream) -> io::Result<()> {
    let rows = 14;
    let cols = 67;
    let delay_factor = 67; // ms

    let mut line = String::new();
    let mut frame = String::from("\x1b[H"); // home

    // BUG: This attempts to read past EOF
    let byte_count = reader.read_line(&mut line)?;
    if byte_count == 0 {
        return Err(std::io::Error::other("movie complete"));
    }
    let delay_num = line.trim().parse::<u64>().unwrap();
    line.clear();

    for _ in 1..rows {
        let byte_count = reader.read_line(&mut line)?;
        if byte_count == 0 {
            return Err(std::io::Error::other("movie truncated"));
        }
        writeln!(frame, "{:cols$}", line.trim_end()).unwrap();
        line.clear();
    }

    stream.write_all(frame.as_bytes())?;

    let delay = delay_factor * delay_num;
    let delay_duration = time::Duration::from_millis(delay);
    thread::sleep(delay_duration);

    Ok(())
}

fn stream_log(msg: &str, stream: &TcpStream) {
    if let Ok(peer_addr) = stream.peer_addr() {
        println!("swserver: {} {}", msg, peer_addr);
    } else {
        println!("swserver: {} [unknown IP]", msg);
    }
}

fn handle_client(mut stream: TcpStream, filename: Arc<String>) {
    stream.set_nonblocking(true).unwrap();

    let file = match File::open(&*filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut reader = BufReader::new(file);

    let _ = stream.write_all(b"\x1b[2J");

    loop {
        if let Err(err) = show_frame(&mut reader, &mut stream) {
            stream_log(&format!("{}", err), &stream);
            break;
        }

        if let Err(e) = consume_input(&mut stream) {
            // client hung up or error
            stream_log(&format!("{}", e), &stream);
            break;
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() == 2 {
        args[1].clone()
    } else if args.len() == 1 {
        "sw1.txt".to_owned()
    } else {
        eprintln!("usage: swserver [infile.txt]");
        process::exit(1);
    };

    let filename = Arc::new(filename);
    let listener = TcpListener::bind("0.0.0.0:2187");

    for stream in listener?.incoming() {
        match stream {
            Ok(stream) => {
                stream_log("new connection", &stream);
                let filename = Arc::clone(&filename);
                thread::spawn(move || handle_client(stream, filename));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
