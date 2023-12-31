pub mod builder;
pub mod constants;
pub mod event;
pub mod models;

/// Re-exported from the futures_util crate.
pub use futures_util::FutureExt as SocketListenerFuture;
/// Re-exported from the socketio crate.
pub use socketio::enums::packet::PacketType;
/// Re-exported from the socketio crate.
pub use socketio::parser::Packet;
/// Re-exported from the socketio crate.
pub use socketio::socket::Socket;
/// Re-exported from the socketio crate.
pub use socketio::socket::SocketReadStream;
/// Re-exported from the socketio crate.
pub use socketio::socket::SocketWriteSink;

pub struct CSGOEmpireSocket {
    socket: Socket,
}

impl CSGOEmpireSocket {
    pub fn new(socket: Socket) -> Self {
        Self { socket }
    }

    pub async fn emit<E, P>(
        &mut self,
        event: E,
        payload: P,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        E: Into<String>,
        P: Into<String>,
    {
        let payload = Packet::new(
            PacketType::Message,
            Some("trade".to_owned()),
            Some(event.into()),
            serde_json::from_str(&payload.into()).ok(),
        );
        let _ = self.socket.send_packet(payload).await;
        Ok(())
    }

    pub async fn emit_custom<P>(&mut self, payload: P) -> Result<(), Box<dyn std::error::Error>>
    where
        P: Into<Packet>,
    {
        self.socket.send_packet(payload.into()).await
    }

    pub async fn emit_raw<P>(&mut self, payload: P) -> Result<(), Box<dyn std::error::Error>>
    where
        P: Into<String>,
    {
        Socket::send_raw(self.socket.write(), payload.into())
            .await
            .map_err(|e| e.into())
    }

    pub fn inner(&self) -> &Socket {
        &self.socket
    }

    pub fn inner_mut(&mut self) -> &mut Socket {
        &mut self.socket
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.handshake().await
    }

    pub async fn reconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!("reconnect")
    }
}
