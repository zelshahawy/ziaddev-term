use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use comfy_table::{presets::UTF8_FULL, Table};
use owo_colors::OwoColorize;

fn terminal_page() -> String {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            "Link".cyan().bold().to_string(),
            "URL".cyan().bold().to_string(),
        ])
        .add_row(vec![
            "GitHub".green().bold().to_string(),
            "https://github.com/zelshahawy".to_string(),
        ])
        .add_row(vec![
            "Website".green().bold().to_string(),
            "https://www.ziadelshahawy.dev/".to_string(),
        ])
        .add_row(vec![
            "Blog".green().bold().to_string(),
            "https://www.ziadelshahawy.dev/blog/".to_string(),
        ]);

    format!(
        "\n{}\n{}\n\n{}\n\n{}\n\n",
        "ziadelshahawy.dev".cyan().bold(),
        "..................".dimmed(),
        table,
        "thanks for visiting :)".magenta()
    )
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let _ = stream.read(&mut buffer);

    let body = terminal_page();

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/plain; charset=utf-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        body.as_bytes().len(),
        body
    );

    let _ = stream.write_all(response.as_bytes());
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .expect("failed to bind to port 8080");

    println!("listening on 0.0.0.0:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(error) => {
                eprintln!("connection failed: {error}");
            }
        }
    }
}
