// Rust Monero Library
// Written in 2019 by
//   h4sh3d <h4sh3d@protonmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//

//! # Consensus
//!
//! This module defines traits and functions which are needed to conform to Monero consensus.
//!
//! ## Encode module
//!
//! The `encode` module is based on Andrew Poelstra work in `rust-bitcoin` library.
//!

#[macro_use]
pub mod encode;

pub use self::encode::{
    deserialize, serialize, serialize_hex, Decodable, Decoder, Encodable, Encoder,
};
