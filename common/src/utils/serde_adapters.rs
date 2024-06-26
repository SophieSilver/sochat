//! Adapters for use with [`serde`] and [`serde_with`]

use std::{cell::Cell, marker::PhantomData, ops::Deref};

use serde::{ser::SerializeSeq, Serialize};
use serde_with::{
    base64::{Alphabet, Base64, UrlSafe},
    formats::{Format, Unpadded},
    Bytes, DeserializeAs, IfIsHumanReadable, Same, SerializeAs,
};
use uuid::Uuid;

/// Serializes byte data as either base64 strings or using optimized byte representation.
///
/// Some crates (e.g. '[serde_json]') don't automatically serialize bytes to base64 strings,
/// even if annotated with `serde(with = "serde_bytes")` or `serde_as(as = "Bytes")`,
/// which results in an inefficient representation.
///
/// However, using base64 exclusively will cause overhead for more optimized binary formats, e.g. `bincode`.
///
/// This type is aimed at striking a compromise.
/// It uses `is_human_readable` method to decide whether to serialize with base64 or with bytes.
pub type BytesOrBase64<ALPHABET = UrlSafe, PADDING = Unpadded> =
    IfIsHumanReadable<Base64<ALPHABET, PADDING>, Bytes>;

/// A type that serializes and deserializes UUIDs or types that can be converted to and from UUIDs using `BytesOrBase64`.
pub struct BytesOrBase64Uuid<ALPHABET: Alphabet = UrlSafe, PADDING: Format = Unpadded> {
    _phantom: PhantomData<(ALPHABET, PADDING)>,
}

impl<T, ALPHABET, PADDING> SerializeAs<T> for BytesOrBase64Uuid<ALPHABET, PADDING>
where
    Uuid: From<T>,
    T: Clone,
    ALPHABET: Alphabet,
    PADDING: Format,
    BytesOrBase64<ALPHABET, PADDING>: SerializeAs<[u8; 16]>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let id: Uuid = source.clone().into();
        let bytes = id.as_bytes();

        BytesOrBase64::<ALPHABET, PADDING>::serialize_as(bytes, serializer)
    }
}

impl<'de, T, ALPHABET, PADDING> DeserializeAs<'de, T> for BytesOrBase64Uuid<ALPHABET, PADDING>
where
    T: From<Uuid>,
    ALPHABET: Alphabet,
    PADDING: Format,
    BytesOrBase64<ALPHABET, PADDING>: DeserializeAs<'de, [u8; 16]>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; 16] = BytesOrBase64::deserialize_as(deserializer)?;
        Ok(Uuid::from_bytes(bytes).into())
    }
}

// TODO: PR this to serde_with

/// A type that allows serializing from a proxy type using a specified adapter.
pub struct FromIntoAndThen<Proxy, Then = Same> {
    _phantom: PhantomData<(Proxy, Then)>,
}

impl<T, Proxy, Then> SerializeAs<T> for FromIntoAndThen<Proxy, Then>
where
    Proxy: From<T>,
    T: Clone,
    Then: SerializeAs<Proxy>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let proxy: Proxy = source.clone().into();
        Then::serialize_as(&proxy, serializer)
    }
}

impl<'de, T, Proxy, Then> DeserializeAs<'de, T> for FromIntoAndThen<Proxy, Then>
where
    T: From<Proxy>,
    Then: DeserializeAs<'de, Proxy>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized_proxy = Then::deserialize_as(deserializer)?;
        Ok(deserialized_proxy.into())
    }
}

/// A wrapper that serializes an iterator into a sequence without collecting it 
/// 
/// Useful when you don't directly hold a slice of the desired type, but can get an iterator
/// of that type and don't want to unnecessarily collect it into a `Vec`.
pub struct SerializeSeqFromIter<I> {
    inner: Cell<Option<I>>,
}

impl<I> SerializeSeqFromIter<I>
where
    I: Iterator,
    I::Item: Deref,
    <I::Item as Deref>::Target: Serialize,
{
    /// Create a new [`SerializeSeqFromIter`] from the give iterator.
    pub fn new(iter: I) -> Self {
        Self {
            inner: Cell::new(Some(iter)),
        }
    }
}

impl<I> Serialize for SerializeSeqFromIter<I>
where
    I: Iterator,
    I::Item: Deref,
    <I::Item as Deref>::Target: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let iter = self
            .inner
            .take()
            .expect("SerializeSeqFromIter should only be serialized once");
        // only set the len to Some if the upper and lower bounds agree
        let (lower_bound, upper_bound) = iter.size_hint();
        let iter_len = upper_bound.filter(|&upper| upper == lower_bound);

        let mut seq = serializer.serialize_seq(iter_len)?;
        for item in iter {
            seq.serialize_element(&*item)?;
        }
        seq.end()
    }
}
