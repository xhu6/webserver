use webserver::Server;

fn main() {
    Server::new().run("0.0.0.0:8000", 4);
}
