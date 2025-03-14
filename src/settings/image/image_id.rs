use core::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// A unique identifier for an image. It is implemented via a SHA-256 hash. This
/// usually can be used to look up the image in an
/// [`ImageCache`](super::ImageCache).
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ImageId(pub(crate) [u8; 32]);

impl fmt::Display for ImageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.format_str(&mut [0; 64]), f)
    }
}

impl fmt::Debug for ImageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.format_str(&mut [0; 64]), f)
    }
}

impl ImageId {
    /// The image ID that represents an empty image (0 bytes of data).
    pub const EMPTY: &'static Self = &Self(*b"\xe3\xb0\xc4\x42\x98\xfc\x1c\x14\x9a\xfb\xf4\xc8\x99\x6f\xb9\x24\x27\xae\x41\xe4\x64\x9b\x93\x4c\xa4\x95\x99\x1b\x78\x52\xb8\x55");

    pub(super) fn hash(&self) -> u64 {
        let [first, _, _, _]: &[[u8; 8]; 4] = bytemuck::cast_ref(&self.0);
        u64::from_ne_bytes(*first)
    }

    /// Returns [`true`] if the image ID represents an empty image (0 bytes of
    /// data).
    pub fn is_empty(&self) -> bool {
        self == Self::EMPTY
    }

    /// Formats the image ID as a string of hex digits. The buffer must be 64
    /// bytes long.
    pub fn format_str<'a>(&self, buf: &'a mut [u8; 64]) -> &'a mut str {
        // FIXME: Use MaybeUninit when it's usable with slices and arrays in
        // stable Rust.
        let mut i = 0;
        for byte in &self.0 {
            buf[i] = format_hex_nibble(byte >> 4);
            buf[i + 1] = format_hex_nibble(byte & 0xF);
            i += 2;
        }
        // SAFETY: We've written hex digits to the entire buffer, so it's valid
        // UTF-8.
        unsafe { core::str::from_utf8_unchecked_mut(buf) }
    }
}

impl FromStr for ImageId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let bytes: &[[u8; 2]; 32] = bytemuck::try_from_bytes(s.as_bytes()).map_err(drop)?;
        let mut dst = [0; 32];
        for (&[a, b], dst) in bytes.iter().zip(&mut dst) {
            *dst = (parse_hex_nibble(a).ok_or(())? << 4) | parse_hex_nibble(b).ok_or(())?;
        }
        Ok(ImageId(dst))
    }
}

impl Default for ImageId {
    fn default() -> Self {
        *Self::EMPTY
    }
}

impl Serialize for ImageId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(self.format_str(&mut [0; 64]))
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl<'de> Deserialize<'de> for ImageId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(ImageIdVisitor)
        } else {
            deserializer.deserialize_bytes(ImageIdVisitor)
        }
    }
}

struct ImageIdVisitor;

impl serde::de::Visitor<'_> for ImageIdVisitor {
    type Value = ImageId;

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ImageId(
            v.try_into().map_err(|_| E::custom("invalid length"))?,
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ImageId::from_str(v).map_err(|_| E::custom("invalid 64 char hex string"))
    }

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a SHA-256 hash")
    }
}

const fn format_hex_nibble(nibble: u8) -> u8 {
    match nibble {
        0..=9 => b'0' + nibble,
        _ => (b'a' - 10) + nibble,
    }
}

const fn parse_hex_nibble(v: u8) -> Option<u8> {
    Some(match v {
        b'0'..=b'9' => v - b'0',
        b'a'..=b'f' => v - (b'a' - 10),
        b'A'..=b'F' => v - (b'A' - 10),
        _ => return None,
    })
}
