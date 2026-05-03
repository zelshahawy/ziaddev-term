use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use comfy_table::{
    Attribute, Cell, Color, Table,
    presets::UTF8_BORDERS_ONLY,
};

fn terminal_page() -> String {
    let mut table = Table::new();

    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_header(vec![
            Cell::new("Link")
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
            Cell::new("URL")
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("GitHub")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new("https://github.com/zelshahawy")
                .fg(Color::Blue),
        ])
        .add_row(vec![
            Cell::new("Website")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new("https://www.ziadelshahawy.dev/")
                .fg(Color::Blue),
        ])
        .add_row(vec![
            Cell::new("Blog")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new("https://www.ziadelshahawy.dev/blog/")
                .fg(Color::Blue),
        ]);

    format!(
        "\n{}\n{}\n\n{}\n\n{}\n",
        "\x1b[1;36mziadelshahawy.dev\x1b[0m",
        "\x1b[2m..................\x1b[0m",
        table,
        "\x1b[35mthanks for visiting :)\x1b[0m"
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
