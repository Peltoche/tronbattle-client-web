use stdweb::web::WebSocket;
use std::sync::Mutex;
use stdweb::web::event::{
    SocketOpenEvent,
    SocketCloseEvent,
    SocketErrorEvent,
    SocketMessageEvent,
};
use stdweb::traits::*;

const CONNECTION: &'static str = "ws://127.0.0.1:8080/socket";

lazy_static! {
    static ref SOCKET: Mutex<WebSocket> = Mutex::new(WebSocket::new(CONNECTION).expect("failed to create the socket"));
}

pub fn init() {
    let socket = SOCKET.lock().expect("failed to lock the socket");

    socket.add_event_listener(|_: SocketOpenEvent| {
        let socket = SOCKET.lock().expect("failed to lock the socket");
        console!(log, "start socket");
        socket.send_text("test").expect("failed to send the test message");
    });

    socket.add_event_listener(|ev: SocketMessageEvent| {
        console!(log, format!("recieve event: {:?}", ev.data()));
    });

    socket.add_event_listener(|ev: SocketErrorEvent| {
        console!(log, format!("connection errors : {:?}", ev));
    });

    socket.add_event_listener(|ev: SocketCloseEvent| {
        console!(log, format!("connection closed : {:?}", ev));
    });
}

