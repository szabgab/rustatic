use std::fs::File;

use tiny_http::{Response, Server};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value = "5000")]
    port: String,

    #[arg(long, default_value = "")]
    indexfile: String,

    #[arg(long, default_value = ".")]
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();
    dbg!(&args.host);

    let server_str = format!("{}:{}", &args.host, &args.port);

    let server = Server::http(&server_str).expect("Failed to start demo server.");
    println!("Visit http://{server_str} Press Ctrl-C to stop the server.");
    for request in server.incoming_requests() {
        //dbg!(request.method());
        dbg!(request.url());
        //dbg!(request.headers());
        let url = &request.url()[1..]; // remove leading slash
        //dbg!(&url);
        //dbg!(&args.path);
        let path = args.path.join(&url);
        //dbg!(&path);
        // if path is a file serve it
        // if path is a directory then
        //     list the directory content
        //     check if the default file is in the directory and server that (e.g. index.html)
        //     return some error (do we need this?)
        // else return error

        if !path.exists() {
            request.respond(Response::from_string("File Not found")).unwrap(); // TODO add 404
        } else if path.is_file() {
            request.respond(Response::from_file(File::open(&path).unwrap())).unwrap(); // TODO set mime-type
        } else if path.is_dir() {
            // TODO if path does not end in / redirect to the same path with /
            if args.indexfile != "" {
                let path = path.join(&args.indexfile);
                if path.exists() && path.is_file() {
                    request.respond(Response::from_file(File::open(&path).unwrap())).unwrap(); // TODO set mime-type
                } else {
                    request.respond(Response::from_string("Could not find indexfile")).unwrap(); // TODO add error
                }
            } else {
            //request.respond(Response::from_file(File::open(&path).unwrap())).unwrap(); // TODO set mime-type
                request.respond(Response::from_string("directory")).unwrap();
            }
        } else {
            request.respond(Response::from_string("We are confused.")).unwrap(); // TODO add 500 error
        }
    }
}

