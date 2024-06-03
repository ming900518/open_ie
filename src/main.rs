#![windows_subsystem = "windows"]
use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    os::windows::process::CommandExt,
    process::Command,
    thread,
};

fn main() {
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], 7000))).unwrap();
    while let Ok((stream, _addr)) = listener.accept() {
        thread::spawn(move || {
            handle_incoming(stream);
        });
    }
}

fn handle_incoming(mut stream: TcpStream) {
    loop {
        let buf_reader = BufReader::new(&mut stream);
        let http_request = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>();
        match http_request.first() {
            Some(method) if method.contains("/?path=") => {
                std::fs::remove_file("init.vbs").ok();
                let path = method
                    .split_once("/?path=")
                    .unwrap_or_default()
                    .1
                    .split_once(" HTTP")
                    .unwrap_or_default()
                    .0
                    .trim();
                std::fs::write(
                    "init.vbs",
                    format!(
                        r#"Set objIE = CreateObject("InternetExplorer.Application")
objIE.Visible = True
CreateObject("WScript.Shell").AppActivate "Internet Explorer"
objIE.Navigate "{path}""#
                    ),
                )
                .ok();
                Command::new("wscript")
                    .arg("init.vbs")
                    .creation_flags(0x00000008)
                    .spawn()
                    .ok();
                let status_line = "HTTP/1.1 200 OK";
                let response = format!(
                    "{status_line}\r\nContent-Length: 0\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n"
                );
                stream.write_all(response.as_bytes()).ok();
            }
            Some(method) if method.contains("/close") => {
                std::fs::remove_file("init.vbs").ok();
                let status_line = "HTTP/1.1 200 OK";
                let response = format!(
                    "{status_line}\r\nContent-Length: 0\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n"
                );
                stream.write_all(response.as_bytes()).ok();
                std::process::exit(0);
            }
            _ => {
                let status_line = "HTTP/1.1 404 Not Found";
                let response = format!(
                    "{status_line}\r\nContent-Length: 0\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n"
                );
                stream.write_all(response.as_bytes()).ok();
            }
        }
    }
}
