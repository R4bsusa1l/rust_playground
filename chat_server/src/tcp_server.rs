use std::net::SocketAddr;
use mio::net::TcpListener;


fn tcp_server(){
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    event_loop.register(&server_socket,
                        Token(0),
                        EventSet::readable(),
                        PollOpt::edge()).unwrap();
}