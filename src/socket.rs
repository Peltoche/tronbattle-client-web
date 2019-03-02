use stdweb::web::event::{
    SocketOpenEvent,
    SocketCloseEvent,
    SocketErrorEvent,
    SocketMessageEvent,
};
use stdweb::traits::*;

use crate::SOCKET;


pub fn register_event_listeners() {
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

