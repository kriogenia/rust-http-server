fn main() {
    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string);
    server.run();
}

struct Server {

    address: String

}

impl Server {

    fn new(address: String) -> Self {
        Self {
            address
        }
    }

    fn run(self) {
        println!("{}",self.address);
    }

}
