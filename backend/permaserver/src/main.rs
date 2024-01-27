use rouille::{Response, router};

fn main() {
    rouille::start_server("127.0.0.1:7878", move |request| {
        println!("{}", request.url());
        let result = router!(request,
            (GET) (/) => {
                "MAIN"
            },
            (GET) (/api/) => {
                "LATEST PERMAVITE API: PAPI V1"
            },
            (GET) (/invite/) => {
                "INVITE"
            },
            (POST) (/api/v1/register_plugin) => {
                "REGISTER"
            },
            _ => "UNKNOWN"
        );

        Response::text(result)
    })
}