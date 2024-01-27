use rouille::Response;

fn main() {
    rouille::start_server("127.0.0.1:7878", move |request| {
        println!("{}", request.raw_query_string());

        Response::text("Hello, World!")
    })
}