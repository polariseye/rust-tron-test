//! A signature signed by some private key.
use std::iter;
use std::{fmt, ops, str};

use hex::{FromHex, ToHex};
use std::convert::TryFrom;

use crate::error::Error;

/// A Signature of Secp256k1. The `v`(rec_id) is different from Ethereum.
#[derive(Clone)]
pub struct Signature([u8; 65]);

impl Signature {
    /// Get a slice into the 'r' portion of the data.
    pub fn r(&self) -> &[u8] {
        &self.0[0..32]
    }

    /// Get a slice into the 's' portion of the data.
    pub fn s(&self) -> &[u8] {
        &self.0[32..64]
    }

    /// Get the recovery byte.
    pub fn v(&self) -> u8 {
        self.0[64]
    }

    /// Check if this is a "low" signature (that s part of the signature is in range
    /// 0x1 and 0x7FFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF 5D576E73 57A4501D DFE92F46 681B20A0 (inclusive)).
    /// This condition may be required by some verification algorithms
    pub fn is_low_s(&self) -> bool {
        const LOW_SIG_THRESHOLD: [u8; 32] = [
            0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x5D, 0x57,
            0x6E, 0x73, 0x57, 0xA4, 0x50, 0x1D, 0xDF, 0xE9, 0x2F, 0x46, 0x68, 0x1B, 0x20, 0xA0,
        ];
        self.s() <= &LOW_SIG_THRESHOLD[..]
    }

    /// As raw signature bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.0[..] == other.0[..]
    }
}

// also manual for the same reason, but the pretty printing might be useful.
impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Signature")
            .field("r", &(&self.0[0..32]).encode_hex::<String>())
            .field("s", &(&self.0[32..64]).encode_hex::<String>())
            .field("v", &(&self.0[64..65]).encode_hex::<String>())
            .finish()
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (&self.0[..]).encode_hex::<String>().fmt(f)
    }
}

impl ops::Deref for Signature {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl str::FromStr for Signature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Vec::from_hex(s)
            .map_err(|_| Error::InvalidSignature)
            .and_then(Signature::try_from)
    }
}

impl TryFrom<&'static str> for Signature {
    type Error = Error;

    fn try_from(s: &'static str) -> Result<Self, Error> {
        s.parse()
    }
}

impl<'a> TryFrom<&'a [u8]> for Signature {
    type Error = Error;

    fn try_from(mut v: &'a [u8]) -> Result<Self, Error> {
        if v.len() > 65 {
            // NOTE: a8f55980f7312adf9bd67b9436d362599cb2f5b83d255435cf5dbdc6bd1eaacd
            // with signature:
            // 2d206c63fad7b7130845c3bfbaf75c057054596619ab5725078e248717c9605a (r)
            // 3c52f496e230a0cdbdf63e443ee7839ceb21ac348e6e69be1470fa37fac89550 (s)
            // 01 (v)
            // 9000 (rubbish surfix)
            v = &v[..65];
        }
        if v.len() == 65 {
            let mut inner = [0u8; 65];
            (&mut inner[..]).copy_from_slice(v);
            // NOTE: 0x2fe5b7a5610aa9dc081574b0451af82ac586ac0a67e2da2a100a5923c862e357
            if inner[64] >= 27 {
                inner[64] -= 27;
            }
            Ok(Signature(inner))
        } else {
            Err(Error::InvalidSignature)
        }
    }
}

impl TryFrom<Vec<u8>> for Signature {
    type Error = Error;

    fn try_from(v: Vec<u8>) -> Result<Self, Error> {
        Signature::try_from(&v[..])
    }
}

impl TryFrom<&Vec<u8>> for Signature {
    type Error = Error;

    fn try_from(v: &Vec<u8>) -> Result<Self, Error> {
        Signature::try_from(&v[..])
    }
}

impl From<[u8; 65]> for Signature {
    fn from(v: [u8; 65]) -> Self {
        Signature(v)
    }
}

impl FromHex for Signature {
    type Error = Error;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        Vec::from_hex(hex.as_ref())
            .map_err(|_| Error::InvalidSignature)
            .and_then(Self::try_from)
    }
}

impl ToHex for Signature {
    fn encode_hex<T: iter::FromIterator<char>>(&self) -> T {
        (&self.0[..]).encode_hex()
    }

    fn encode_hex_upper<T: iter::FromIterator<char>>(&self) -> T {
        (&self.0[..]).encode_hex_upper()
    }
}

impl From<Signature> for Vec<u8> {
    fn from(s: Signature) -> Self {
        (&s.0[..]).to_owned()
    }
}
