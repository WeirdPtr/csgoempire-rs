use super::{constants::SOCKET_ADDRESS, event::CSGOEmpireSocketEvent, CSGOEmpireSocket};
use crate::{
    api::CSGOEmpireApi,
    util::{base64_encode, crate_user_agent},
};
use futures_util::{future::BoxFuture, lock::Mutex, FutureExt};
use reqwest::header::HeaderMap;
use serde_json::json;
use socketio::{
    enums::packet::PacketType,
    parser::Packet,
    socket::{builder::SocketBuilder, Socket, SocketReadStream, SocketWriteSink},
    Request,
};
use std::{sync::Arc, thread};

pub struct CSGOEmpireSocketBuilder {
    builder: SocketBuilder,
    api_key: &'static str,
    address: &'static str,
    headers: Option<HeaderMap>,
    max_price: Option<i64>,
}

impl CSGOEmpireSocketBuilder {
    pub fn new<K>(api_key: K) -> Self
    where
        K: Into<&'static str>,
    {
        let mut builder = SocketBuilder::new(Self::generate_inner_request(SOCKET_ADDRESS, None));

        let api_key = api_key.into();

        Self::prepare_builder(&mut builder, api_key);

        Self {
            api_key,
            builder,
            address: SOCKET_ADDRESS,
            headers: None,
            max_price: None,
        }
    }

    pub fn new_with_address<K, A>(api_key: K, address: A) -> Self
    where
        K: Into<&'static str>,
        A: Into<&'static str>,
    {
        let address = address.into();

        let mut builder = SocketBuilder::new(Self::generate_inner_request(address, None));

        let api_key = api_key.into();

        Self::prepare_builder(&mut builder, api_key);

        Self {
            api_key,
            address,
            builder,
            headers: None,
            max_price: None,
        }
    }

    fn generate_key() -> String {
        let nonce: [u8; 16] = rand::random();
        base64_encode(&nonce)
    }

    fn get_request_from_uri(
        address: &str,
        headers: Option<&HeaderMap>,
    ) -> Result<Request<()>, Box<dyn std::error::Error>> {
        let captures = regex::Regex::new(r"(https?|wss?)://([^/]+)/")?.captures(address);

        if captures.is_none() {
            return Err("Malformed address")?;
        }

        let host = captures.unwrap().get(2);

        if host.is_none() {
            return Err("Malformed address")?;
        }

        let mut request = Request::builder()
            .uri(address)
            .header("Host", host.unwrap().as_str())
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", Self::generate_key())
            .header("Upgrade", "websocket")
            .header("Connection", "upgrade")
            .header("User-Agent", crate_user_agent());

        if let Some(headers) = headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }

        let request = request.body(())?;

        Ok(request)
    }

    fn generate_inner_request(url: &str, headers: Option<&HeaderMap>) -> Request<()> {
        Self::get_request_from_uri(url, headers).unwrap_or(
            Self::get_request_from_uri(SOCKET_ADDRESS, headers).unwrap_or(
                Self::get_request_from_uri(SOCKET_ADDRESS, None).expect("Malformed address"),
            ),
        )
    }

    fn rebuild_inner(&mut self, url: &str) -> &mut Self {
        self.builder = SocketBuilder::new(Self::generate_inner_request(url, self.headers.as_ref()));

        Self::prepare_builder(&mut self.builder, self.api_key);

        self
    }

    fn prepare_builder(builder: &mut SocketBuilder, api_key: &'static str) {
        builder
        .on("handshake", move |_, _, write| {
            async move {
                let _ = Socket::send_raw_packet(
                    write.clone(),
                    Packet::new(PacketType::Ping, Some("trade".to_owned()), None, None),
                )
                .await;

                let _ = Socket::send_raw_packet(
                    write.clone(),
                    Packet::new(PacketType::Event, None, None, None),
                )
                .await;
            }
            .boxed()
        })
        .on("init", move|packet, _, write| {
            async move {
                // TODO: rewrite this
                let is_authenticated = packet.data.unwrap_or(json!({"authenticated": false}));
                let is_authenticated = is_authenticated.get("authenticated");

                if is_authenticated.is_none() {
                    return;
                }

                let is_authenticated = is_authenticated.unwrap().as_bool();

                if is_authenticated.is_none()  {
                    return;
                }

                if is_authenticated.unwrap() {
                    // let _ = Socket::send_raw_packet(write,Packet::new(
                    //     PacketType::Message,
                    //     Some("trade".to_owned()),
                    //     Some("filters".to_owned()),
                    //     Some(json!({"price_max": 9999999})),
                    // ))
                    // .await;

                    let _ = Socket::send_raw(write, r#"42/trade,["filters",{"price_max":9999999}]"#.to_owned()).await;
                    return;
                }

                let user_metadata = CSGOEmpireApi::metadata(api_key).send().await;

                if user_metadata.is_err() {
                    return;
                }

                let user_metadata = user_metadata.unwrap();

                let socket_token = user_metadata.socket_token.to_owned();

                if socket_token.is_none() {
                    return;
                }

                let socket_token = socket_token.unwrap();

                let socket_signature = user_metadata.socket_signature;

                if socket_signature.is_none() {
                    return;
                }

                let socket_signature = socket_signature.unwrap();

                let user_model = user_metadata.user.as_ref();

                if user_model.is_none() {
                    return;
                }

                let user_model = user_model.unwrap();

                let user_id = user_model.id;

                let user_model = serde_json::to_string(&user_model);

                if user_model.is_err() {
                    return;
                }

                let user_model = user_model.unwrap().replace("\\\"", "\"");

                let raw_packet = format!(r#"42/trade,["identify",{{"uid":{user_id},"model":{user_model},"authorizationToken":"{socket_token}","signature":"{socket_signature}"}}]"#);

                let _ = Socket::send_raw(write, raw_packet).await;



            }
            .boxed()
        });
    }

    /// <b> Warning: This will clear all set event handlers </b>
    pub fn change_url(&mut self, url: &str) -> &mut Self {
        self.rebuild_inner(url);
        self
    }

    pub fn on<L>(&mut self, event: CSGOEmpireSocketEvent, listener: L) -> &mut Self
    where
        L: for<'a> Fn(
                Packet,
                Arc<Mutex<SocketReadStream>>,
                Arc<Mutex<SocketWriteSink>>,
            ) -> BoxFuture<'static, ()>
            + 'static
            + Send
            + Sync,
    {
        self.builder.on(event, listener);
        self
    }

    pub fn on_any<L>(&mut self, listener: L) -> &mut Self
    where
        L: for<'a> Fn(
                Packet,
                Arc<Mutex<SocketReadStream>>,
                Arc<Mutex<SocketWriteSink>>,
            ) -> BoxFuture<'static, ()>
            + 'static
            + Send
            + Sync,
    {
        self.builder.on_any(listener);
        self
    }

    pub async fn build(self) -> Result<CSGOEmpireSocket, Box<dyn std::error::Error>> {
        let mut socket_instance = CSGOEmpireSocket::new_with_url(
            self.api_key,
            self.address,
            self.builder.connect().await?,
        );

        // socket_instance.socket.handshake().await?;

        socket_instance.emit_raw("40/trade,").await?;

        Ok(socket_instance)
    }
}
