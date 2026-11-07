//! SigmaOS-base64 — Base64 encoding/decoding model.
//!
//! Models base64 encoding and decoding operations.
//! Deterministic and std-only.

pub mod codec;
pub mod options;

pub use codec::{Base64Codec, DecodeResult};
pub use options::{Base64Options, WrapMode};
