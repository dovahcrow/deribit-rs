#![feature(async_await, futures_api, await_macro)]

use deribit::models::HelloRequest;
use deribit::Deribit;
use env_logger::init;
use futures::compat::Compat;
use futures::FutureExt;

fn main() {
    init();

    let drb = Deribit::new();
    let fut = async {
        let mut client = match await!(drb.connect()) {
            Ok((client, _)) => client,
            Err(e) => {
                println!("Err1 {:?}", e);
                return;
            }
        };

        let req = HelloRequest {
            client_name: "deribit-rs".into(),
            client_version: "0.0.1".into(),
        };

        let resp = match await!(client.hello(&req)) {
            Ok(v) => v,
            Err(e) => {
                println!("Err2 {:?}", e);
                return;
            }
        };

        println!("{:?}", resp);
    };
    let fut = Compat::new(fut.boxed().map(|_| Ok(())));
    tokio::run(fut);
}
