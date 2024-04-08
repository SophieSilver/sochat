use bytes::Bytes;
use common::types::{Id, MessageId, UnreadMessage, UserId};

// TODO: rework everything about this
// TODO: have a custom error type
// TODO: have a thing that retries on certain kinds of errors
// TODO:  make it changeable
// TODO: check status code

const HOST: &str = "http://127.0.0.1:11800";

pub async fn register_user() -> eyre::Result<UserId> {
    let response = reqwest::Client::new()
        //.post(reqwest::Url::from_str(HOST).unwrap().join("users").unwrap())
        .post(format!("{HOST}/users"))
        .send()
        .await?;

    let bytes = response.bytes().await?;

    let id = UserId::from_bytes(&bytes)?;

    Ok(id)
}

pub async fn send_message(from: UserId, to: UserId, content: impl Into<Bytes>) -> eyre::Result<()> {
    reqwest::Client::new()
        .post(format!("{HOST}/messages/from/{from}/to/{to}"))
        .body(content.into())
        .send()
        .await?;

    Ok(())
}

pub async fn fetch_unread_messages(to: UserId, from: UserId) -> eyre::Result<Box<[UnreadMessage]>> {
    let response = reqwest::Client::new()
        .get(format!("{HOST}/messages/from/{from}/to/{to}"))
        .send()
        .await?;

    let messages = ciborium::from_reader(&response.bytes().await?[..])?;
    Ok(messages)
}

pub async fn mark_received(
    to: UserId,
    from: UserId,
    // TODO: accept iterator instead
    ids: &[MessageId],
) -> eyre::Result<()> {
    let mut buf = Vec::<u8>::new();
    ciborium::into_writer(&ids, &mut buf)?;

    reqwest::Client::new()
        .post(format!("{HOST}/messages/from/{from}/to/{to}/received"))
        .header(reqwest::header::CONTENT_TYPE, "application/cbor")
        .body(buf)
        .send()
        .await?;

    Ok(())
}
