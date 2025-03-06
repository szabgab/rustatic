use std::env;
use std::fs::File;
use std::path::Path;
use std::str::FromStr as _;

use ascii::AsciiString;
use clap::Parser;
use mime_guess::mime;
use tiny_http::{Header, HeaderField, Response, Server, StatusCode};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[expect(
    clippy::arbitrary_source_item_ordering,
    reason = "host and port are related and thus should be together"
)]
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

/// if path is a file serve it
/// if path is a directory then
///     list the directory content
///     check if the default file is in the directory and server that (e.g. index.html)
///     return some error (do we need this?)
/// else return error
fn main() {
    let args = Cli::parse();
    //dbg!(&args.host);

    let server_str = format!("{}:{}", &args.host, &args.port);

    let Ok(server) = Server::http(&server_str) else {
        println!("Could not start the server. Is the port maybe already taken?");
        std::process::exit(1);
    };

    println!("Running version {VERSION}");
    println!("Visit http://{server_str} Press Ctrl-C to stop the server.");
    for request in server.incoming_requests() {
        //dbg!(request.method());
        print!("request: {} ", request.url());
        //dbg!(request.headers());
        let url = &request.url()[1..]; // remove leading slash
                                       //dbg!(&url);
                                       //dbg!(&args.path);
        let url_path = if url.contains('?') {
            url.split('?').next().unwrap()
        } else {
            url
        };
        let path = args.path.join(url_path);
        let mut p = path.clone().into_os_string();
        p.push(".html");
        let html_path: std::path::PathBuf = p.into();

        // dbg!(&path);
        // dbg!(&html_path);
        // dbg!(&args.nice);

        if args.nice && html_path.is_file() {
            println!("200 OK {}", html_path.metadata().unwrap().len());
            request
                .respond(Response::from_file(File::open(&html_path).unwrap()))
                .unwrap();
        } else if !path.exists() {
            println!("404 File {} Not Found", path.display());
            request
                .respond(
                    Response::from_string("File Not found").with_status_code(StatusCode::from(404)),
                )
                .unwrap();
        } else if path.is_file() {
            println!(
                "200 OK {} {}",
                path.metadata().unwrap().len(),
                path.display()
            );
            request.respond(send_file(&path)).unwrap();
        } else if path.is_dir() {
            // if path does not end in / redirect to the same path with /
            if !request.url().ends_with('/') {
                let response = redirect_with_trailing_slash(&request);
                println!("301 Redirect with trailing slash");
                request.respond(response).unwrap();
            } else if !args.indexfile.is_empty() {
                let file_path = path.join(&args.indexfile);
                if file_path.exists() && file_path.is_file() {
                    println!("200 OK Indexfile {}", file_path.metadata().unwrap().len());
                    request
                        .respond(Response::from_file(File::open(&file_path).unwrap()))
                        .unwrap();
                } else {
                    println!("404 Indexfile File {} Not Found", file_path.display());
                    request
                        .respond(
                            Response::from_string("Could not find indexfile")
                                .with_status_code(StatusCode::from(404)),
                        )
                        .unwrap();
                }
            } else {
                println!("200 OK Directory listing");
                let response = directory_listing(&path);
                request.respond(response).unwrap();
            }
        } else {
            println!("500 We are confused");
            request
                .respond(
                    Response::from_string("We are confused.")
                        .with_status_code(StatusCode::from(500)),
                )
                .unwrap();
        }
    }
}

fn directory_listing(path: &Path) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut html = String::new();
    html += "<ul>";
    let Ok(dh) = path.read_dir() else {
        return Response::from_string("Could not read content of the folder.")
            .with_status_code(StatusCode::from(500));
    };

    for entry in dh.flatten() {
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
    html += "</ul>";
    // dbg!(&html);
    let header = Header {
        field: HeaderField::from_str("Content-type").unwrap(),
        value: AsciiString::from_ascii("text/html").unwrap(),
    };
    Response::from_string(&html).with_header(header)
}

fn redirect_with_trailing_slash(
    request: &tiny_http::Request,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let new_url = format!("{}/", request.url());
    let header = Header {
        field: HeaderField::from_str("Location").unwrap(),
        value: AsciiString::from_ascii(new_url).unwrap(),
    };
    Response::from_string("dir")
        .with_status_code(StatusCode::from(301))
        .with_header(header)
}

fn send_file(path: &std::path::PathBuf) -> Response<File> {
    let content_type = mime_guess::from_path(path)
        .first()
        .unwrap_or(mime::TEXT_HTML)
        .to_string();

    let header = Header {
        field: HeaderField::from_str("Content-type").unwrap(),
        value: AsciiString::from_ascii(content_type).unwrap(),
    };
    Response::from_file(File::open(path).unwrap()).with_header(header)
}
