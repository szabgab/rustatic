use tiny_http::{Response, Server};

fn main() {
    let server = Server::http("127.0.0.1:3030").expect("Failed to start demo server.");
    println!("Visit http://localhost:3030 Press Ctrl-C to stop the server.");
    for req in server.incoming_requests() {
        req.respond(Response::from_string("Hello World")).unwrap();
    }
}
