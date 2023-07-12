use tiny_http::{Response, Server};

fn main() {
    let port = "5000";
    let host = "127.0.0.1";
    let server_str = format!("{host}:{port}");

    let server = Server::http(&server_str).expect("Failed to start demo server.");
    println!("Visit http://{server_str} Press Ctrl-C to stop the server.");
    for request in server.incoming_requests() {
        //dbg!(request.method());
        dbg!(request.url());
        //dbg!(request.headers());
        request.respond(Response::from_string("Hello World")).unwrap();
    }
}
