use std::env;
use webserver::Server;

fn main() {
    let args: Vec<_> = env::args().collect();
    let addr = args.get(1).map(|x| x.as_str()).unwrap_or(":::8000");
    let threads = args
        .get(2)
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(4);

    println!("Running on {addr} with {threads} threads");

    let res = Server::new().run(addr, threads);

    println!("Usage: {} [addr] [threads]", args[0]);
    println!("{res:?}");
}
