use warp::Filter;

use serde_derive::{Deserialize, Serialize};

/*
mod data;
mod db;
mod error;
mod handler;
*/

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

#[tokio::main]
async fn main() {
    //pretty_env_logger::init();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");

    // How about multiple segments? First, we could use the `path!` macro:
    //
    // GET /hello/from/warp
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");

    // Fine, but how do I handle parameters in paths?
    //
    // GET /sum/:u32/:u32
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    // What! And? What's that do?
    //
    // It combines the filters in a sort of "this and then that" order. In
    // fact, it's exactly what the `path!` macro has been doing internally.
    //
    // GET /bye/:string
    let bye = warp::path("bye")
        .and(warp::path::param())
        .map(|name: String| format!("Good bye, {}!", name));

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("employees"))
        .and(warp::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            warp::reply::json(&employee)
        });

    let routes = warp::get().and(hi.or(hello_from_warp).or(bye).or(sum));

    /*
    warp::serve(promote)
        .run(([0, 0, 0, 0], 3031))
        .await;
    */

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}