use std::env;
use std::fs::File;
use std::str::FromStr;

use ascii::AsciiString;
use tiny_http::{Response, Server, StatusCode, Header, HeaderField};
use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value = "5000")]
    port: String,

    #[arg(long, default_value = "")]
    indexfile: String,

    #[arg(long)]
    nice: bool,

    #[arg(long, default_value = ".")]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    dbg!(&args.host);

    let server_str = format!("{}:{}", &args.host, &args.port);

    let server = Server::http(&server_str).expect("Failed to start demo server.");
    println!("Running version {VERSION}");
    println!("Visit http://{server_str} Press Ctrl-C to stop the server.");
    for request in server.incoming_requests() {
        //dbg!(request.method());
        dbg!(request.url());
        //dbg!(request.headers());
        let url = &request.url()[1..]; // remove leading slash
        //dbg!(&url);
        //dbg!(&args.path);
        let path = args.path.join(&url);
        let mut p = path.clone().into_os_string();
        p.push(".html");
        let html_path :std::path::PathBuf = p.into();

        dbg!(&path);
        dbg!(&html_path);
        dbg!(&args.nice);
        // if path is a file serve it
        // if path is a directory then
        //     list the directory content
        //     check if the default file is in the directory and server that (e.g. index.html)
        //     return some error (do we need this?)
        // else return error

        if args.nice && html_path.is_file() {
            request.respond(Response::from_file(File::open(&html_path).unwrap())).unwrap();
        } else if !path.exists() {
            request.respond(Response::from_string("File Not found").with_status_code(StatusCode::from(404))).unwrap();
        } else if path.is_file() {
            request.respond(Response::from_file(File::open(&path).unwrap())).unwrap();
        } else if path.is_dir() {
            // if path does not end in / redirect to the same path with /
            if !request.url().ends_with("/") {
                dbg!("fixing path");
                let new_url = format!("{}/", request.url());
                let header = Header {field: HeaderField::from_str("Location").unwrap(), value: AsciiString::from_ascii(new_url).unwrap()};
                request.respond(Response::from_string("dir").with_status_code(StatusCode::from(301)).with_header(header)).unwrap();
            } else if args.indexfile != "" {
                let path = path.join(&args.indexfile);
                if path.exists() && path.is_file() {
                    request.respond(Response::from_file(File::open(&path).unwrap())).unwrap();
                } else {
                    request.respond(Response::from_string("Could not find indexfile").with_status_code(StatusCode::from(404))).unwrap();
                }
            } else {
                let mut html = "".to_string();
                html += "<ul>";
                for entry in path.read_dir().expect("read_dir call failed") {
                    if let Ok(entry) = entry {
                        if entry.path().is_file() || entry.path().is_dir() {
                            html += "<li><a href=\"";
                            html += &entry.file_name().into_string().unwrap();
                            html += "\">";
                            html += &entry.file_name().into_string().unwrap();
                            html += "</a>";
                            html += "</li>";
                        } else {
                            html += "<li>";
                            html += &entry.file_name().into_string().unwrap();
                            html += "</li>";
                        }
                    }
                }
                html += "</ul>";
                dbg!(&html);
                let header = Header {field: HeaderField::from_str("Content-type").unwrap(), value: AsciiString::from_ascii("text/html").unwrap()};
                request.respond(Response::from_string(&html).with_header(header)).unwrap();
            }
        } else {
            request.respond(Response::from_string("We are confused.").with_status_code(StatusCode::from(500))).unwrap();
        }
    }
}

