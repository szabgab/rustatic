use tiny_http::{Response, Server};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value = "5000")]
    port: String,

    #[arg(default_value = ".")]
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
        request.respond(Response::from_string("Hello World")).unwrap();
    }
}
