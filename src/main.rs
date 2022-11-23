use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;

mod handlers;
mod routes;


#[tokio::main]
async fn main()  {


    let customer_routes = routes::create_routes();

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(customer_routes).run((Ipv4Addr::LOCALHOST, port)).await;

}
