use super::{constants::SOCKET_ADDRESS, event::CSGOEmpireSocketEvent, CSGOEmpireSocket};
use crate::{
    api::CSGOEmpireApi,
    util::{base64_encode, crate_user_agent},
};
use futures_util::{future::BoxFuture, lock::Mutex, FutureExt};
use http::header::HeaderMap;
use serde_json::json;
use socketio::{
    get_empty_body,
    parser::Packet,
    socket::{builder::SocketBuilder, Socket, SocketReadStream, SocketWriteSink},
    EmptyRequest, Request,
};
use std::sync::Arc;

pub struct CSGOEmpireSocketBuilder<'a> {
    builder: SocketBuilder,
    api_key: &'a str,
    headers: Option<HeaderMap>,
}

impl<'k> CSGOEmpireSocketBuilder<'k> {
    pub fn new<K>(api_key: impl Into<&'k str>) -> Self {
        Self::new_with_address(api_key, SOCKET_ADDRESS)
    }

    pub fn new_with_address(api_key: impl Into<&'k str>, address: impl Into<&'k str>) -> Self {
        let address = address.into();

        let inner = Self::generate_inner_request(address, None);

        let mut builder = SocketBuilder::new_with_request(inner);

        let api_key = api_key.into();

        Self::prepare_builder(&mut builder, api_key);

        Self {
            api_key,
            builder,
            headers: None,
        }
    }

    fn generate_key() -> String {
        let nonce: [u8; 16] = rand::random();
        base64_encode(&nonce)
    }

    fn get_request_from_uri(
        address: &str,
        headers: Option<&HeaderMap>,
    ) -> Result<EmptyRequest, Box<dyn std::error::Error>> {
        let captures = regex::Regex::new(r"(https?|wss?)://([^/:]+)")?.captures(address);

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

        request.body(get_empty_body()).map_err(|e| e.into())
    }

    fn generate_inner_request(url: &str, headers: Option<&HeaderMap>) -> EmptyRequest {
        Self::get_request_from_uri(url, headers).unwrap_or(
            Self::get_request_from_uri(SOCKET_ADDRESS, headers).unwrap_or(
                Self::get_request_from_uri(SOCKET_ADDRESS, None).expect("Malformed address"),
            ),
        )
    }

    fn rebuild_inner(&mut self, url: &str) -> &mut Self {
        self.builder = SocketBuilder::new_with_request(Self::generate_inner_request(
            url,
            self.headers.as_ref(),
        ));

        Self::prepare_builder(&mut self.builder, self.api_key);

        self
    }

    fn prepare_builder(builder: &mut SocketBuilder, api_key: impl Into<String>) {
        let api_key = Arc::new(api_key.into());

        builder
        .on("init", move|packet, _, write| {
            let api_key = api_key.clone();
            async move {
               let default_auth = json!({"authenticated": false});

                // TODO: rewrite this
                let is_authenticated = packet.data.as_ref().unwrap_or(&default_auth);
                let is_authenticated = is_authenticated.get("authenticated");

                if is_authenticated.is_none() {
                    return;
                }

                let is_authenticated = match is_authenticated.unwrap().as_bool() {
                    Some(value) => value,
                    None => return
                };

                //      {
                //         let _ = Socket::send_raw_packet(
                //             write.clone(),
                //             Packet::new(PacketType::Ping, Some("trade".to_owned()), None, None),
                //         )
                //         .await;

                //         let _ = Socket::send_raw_packet(
                //             write.clone(),
                //             Packet::new(PacketType::Event, None, None, None),
                //         )
                //         .await;
                //     },


                if is_authenticated {
                    drop(tokio::spawn(Socket::send_raw(write, br#"42/trade,["filters",{"price_max":9999999}]"# as &[u8])));
                    return;
                }

                let user_metadata = CSGOEmpireApi::metadata(api_key.as_ref()).send().await;

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

                let _ = Socket::send_raw(write, raw_packet.as_bytes()).await;
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
                Arc<Packet>,
                Arc<Mutex<SocketReadStream>>,
                Arc<Mutex<SocketWriteSink>>,
            ) -> BoxFuture<'static, ()>
            + 'static
            + Send
            + Sync,
    {
        match event {
            CSGOEmpireSocketEvent::Disconnect => self.builder.on("close", listener),
            _ => self.builder.on(event, listener),
        };

        self
    }

    pub fn on_any<L>(&mut self, listener: L) -> &mut Self
    where
        L: for<'a> Fn(
                Arc<Packet>,
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
        let mut socket_instance = CSGOEmpireSocket::new(self.builder.connect().await?);

        socket_instance.emit_raw(br"40/trade," as &[u8]).await?;

        Ok(socket_instance)
    }

    pub fn ignore_invalid_proxy(&mut self, value: bool) -> &mut Self {
        self.builder.ignore_invalid_proxy(value);
        self
    }

    pub fn inner(&self) -> &SocketBuilder {
        &self.builder
    }

    pub fn inner_mut(&mut self) -> &mut SocketBuilder {
        &mut self.builder
    }
}
