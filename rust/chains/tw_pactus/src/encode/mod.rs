// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use std::io::{self, Cursor};

use error::Error;
use tw_memory::Data;

// use crate::encode::stream::Stream;

pub mod var_int;
pub mod encode;
pub mod decode;
pub mod error;

pub fn serialize<T: Encodable>(t: &T) -> Result<Data, std::io::Error>
{
    let mut writer = Vec::with_capacity(t.encoded_size());
    t.encode(&mut writer)?;

    Ok(writer.to_vec())
}

pub fn deserialize<T: Decodable>(data: &[u8]) -> Result<T, Error>
{
    let mut cursor = Cursor::new(data);
    T::decode(&mut cursor)
}

/// Trait for encoding an object into a consistent byte sequence.
pub trait Encodable {
    /// Encode the object in consistent and deterministic way.
    fn encode<W: std::io::Write + ?Sized>(&self, w: &mut W) -> Result<usize, io::Error>;

    /// Determine the size of serialized object.
    fn encoded_size(&self) -> usize;
}

/// Trait for decoding an object from a byte sequence.
pub trait Decodable: Sized {
    /// Decode the object in consistent and deterministic way.
    fn decode(r: &mut dyn std::io::Read) -> Result<Self, Error>;
}
