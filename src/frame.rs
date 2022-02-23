use bytes::{Bytes, BytesMut};
use rand::{thread_rng, RngCore};

pub const MAX_FRAME_LEN: usize = 65536;
pub const HEADER_LEN: usize = 4;
pub const NONCE_LEN: usize = 8;

// FIXME: Finish Frame logic and write header / message parser

#[derive(Debug)]
pub struct Frame {
    #[allow(dead_code)]
    header: Header,
    #[allow(dead_code)]
    message: Message,
}

#[derive(Debug)]
pub struct Header {
    /// Protocol version and direction of frame (request/response)
    pub version: u8,
    /// `u8` opcode that maps to a known command
    pub opcode: u8,
    /// `u16` field to determine length of the body
    pub length: u16,
    /// `u64` randomly generated bytes that are used as a unique identifier
    pub nonce: u64,
}

#[derive(Debug)]
pub struct Message {
    /// Variable length payload determined by the `length_field` value
    pub body: BytesMut,
}

/// The protocol version, coupled with the direction of the frame.
///
/// The "direction" of the frame indicates whether the frame is a `Request` or
/// a `Response` variant. This will help keep track of traffic flow later.
#[derive(Debug)]
pub enum Version {
    Request,
    Response,
}

#[derive(Debug)]
pub struct Nonce(Bytes);

impl Nonce {
    /// Constructs a new `Nonce` by generating a random array of bytes.
    pub fn new() -> Self {
        let mut dst = [0_u8; 8];
        thread_rng().fill_bytes(&mut dst);
        Self(Bytes::copy_from_slice(&dst))
    }

    /// Get a reference to the inner data
    pub fn as_bytes(&self) -> &Bytes {
        &self.0
    }
}

impl Default for Nonce {
    fn default() -> Self {
        Self::new()
    }
}
