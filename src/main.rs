use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server berjalan di http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Gagal menerima koneksi: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("Kesalahan membaca request: {}", e);
            return;
        }
        None => {
            eprintln!("Tidak ada data yang diterima dari klien.");
            return;
        }
    };

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10)); // Simulate a slow request
            ("HTTP/1.1 200 OK", "static/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "static/error.html"),
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        format!("<html><body><h1>File {} tidak ditemukan</h1></body></html>", filename)
    });

    let response = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents);

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Gagal menulis respons: {}", e);
    }
}