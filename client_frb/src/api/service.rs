use anyhow::anyhow;
use client_lib::{
    common::types::{UnreadMessage, UserId},
    http_utils::ClientExt,
    message_receiver::MessageReceiver,
    reqwest::Client,
    server_connection::ServerConnection,
};
use derivative::Derivative;
use flutter_rust_bridge::frb;
use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};
use tokio::runtime::Handle as TokioHandle;

use crate::frb_generated::StreamSink;

#[frb(ignore)]
#[derive(Debug, Default)]
struct ServiceState {
    messages: HashMap<(UserId, UserId), Vec<String>>,
    message_receiver: Option<MessageReceiver>,
}

// TODO: clean up the struct
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Service {
    this_id: UserId,
    tokio_handle: TokioHandle,
    connection: ServerConnection,
    state: Arc<Mutex<ServiceState>>,
}

impl Service {
    /// Do not call directly
    ///
    /// Initialize Service
    #[frb(name = "internal_init")]
    pub async fn init() -> anyhow::Result<Self> {
        let connection = ServerConnection::new(Client::sochat_new()?);
        let user_id = connection.register_user().await?;
        let state: Arc<Mutex<ServiceState>> = Default::default();

        Ok(Self {
            this_id: user_id,
            connection,
            tokio_handle: TokioHandle::current(),
            state,
        })
    }

    /// Do not call directly
    ///
    /// Initialize the message stream
    #[frb(name = "internal_init_message_stream")]
    pub fn init_message_stream(&self, message_notification_sink: StreamSink<()>) {
        // so that MessageReceiver doesn't panic
        let _g = self.tokio_handle.enter();

        let this = self.clone();
        let message_receiver =
            MessageReceiver::new(this.connection.clone(), this.this_id, move |res| {
                match res {
                    Ok(UnreadMessage {
                        id: _id,
                        sender_id,
                        content,
                    }) => {
                        this.store_message(
                            &sender_id,
                            &this.this_id,
                            String::from_utf8(content.into()).unwrap(),
                        );
                        message_notification_sink
                            .add(())
                            .expect("Sending data to Dart must not fail");
                    }
                    Err(e) => message_notification_sink
                        .add_error(anyhow!(e))
                        .expect("Sending data to Dart must not fail"),
                };
            });

        self.inner().message_receiver = Some(message_receiver);
    }

    #[frb(sync, getter)]
    pub fn this_id(&self) -> UserId {
        self.this_id
    }

    fn inner(&self) -> impl DerefMut<Target = ServiceState> + '_ {
        self.state.lock().unwrap()
    }

    // IMPORTANT!!!
    // MAKE SURE TO TAKE IDs by reference,
    // This is because if you take them by value
    // FRB will move them out of Arcs and dispose of the Arcs
    // Causing "Arc" used after being disposed errors

    #[frb(sync)]
    pub fn message_count(&self, from: &UserId, to: &UserId) -> i64 {
        self.inner()
            .messages
            .get(&(*from, *to))
            .map(|v| v.len())
            .unwrap_or(0) as _
    }

    #[frb(sync)]
    pub fn get_message(&self, from: &UserId, to: &UserId, index: i64) -> Option<String> {
        self.inner()
            .messages
            .get(&(*from, *to))?
            .get(usize::try_from(index).unwrap())
            .cloned()
    }

    #[frb(sync)]
    pub fn send_message(&self, to: &UserId, message: String) {
        dbg!(&self);
        self.store_message(&self.this_id, to, message);
    }

    fn store_message(&self, from: &UserId, to: &UserId, message: String) {
        self.inner()
            .messages
            .entry((*from, *to))
            .or_default()
            .push(message);
    }
}
