use std::path::Path;

use micro_http::{HttpServer, Response, StatusCode, EndpointHandler, HttpRoutes, Method};

struct HandlerArg(bool);
struct FooHandler{}

impl EndpointHandler<HandlerArg> for FooHandler {
    fn handle_request(&self, _req: &micro_http::Request, _arg: &HandlerArg) -> Response {
        Response::new(micro_http::Version::Http11, StatusCode::NoContent)
    }
}

fn main() {

    let mut router = HttpRoutes::new("foo_server".to_string(), "/api/v1/".to_string());
    let handler = FooHandler{};
    router.add_route(Method::Get, "foo".to_string(), Box::new(handler)).expect("failed to add router");

    let uds = "./test.sock";
    if Path::new(uds).exists() {
        std::fs::remove_file(uds).expect("failed to remove uds");
    }
    let mut s = HttpServer::new(uds).unwrap();
    s.start_server().expect("failed to start server");
    println!("start server");

    loop {
        for request in s.requests().unwrap() {
            println!("got request");
            let arg = HandlerArg(true);
            let response = request.process(|request| {
                let resp = router.handle_http_request(request, &arg);
                println!("content length {}",resp.content_length());
                // let response_body = b"foo";
                // resp.set_body(Body::new(response_body.to_vec()));
                resp
            });
            println!("got response {:?}", response);
            s.respond(response).expect("failed to response");
        }

        // println!("exit loop");
        // Break this example loop.
        // break;
    }
}
