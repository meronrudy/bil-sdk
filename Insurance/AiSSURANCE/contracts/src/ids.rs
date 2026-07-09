//! Stable identity primitives used across telemetry, risk events, and claims.

use core::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! id_type {
    ($name:ident, $test_tag:expr) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub [u8; 16]);

        impl $name {
            pub fn new() -> Self {
                Self(*Uuid::new_v4().as_bytes())
            }

            pub const fn from_bytes(bytes: [u8; 16]) -> Self {
                Self(bytes)
            }

            pub const fn as_bytes(&self) -> &[u8; 16] {
                &self.0
            }

            pub const fn test_id(index: u8) -> Self {
                let mut bytes = [0u8; 16];
                bytes[0] = index;
                bytes[15] = $test_tag;
                Self(bytes)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({:02x?})", stringify!($name), self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self, f)
            }
        }
    };
}

id_type!(MachineId, 1);
id_type!(SiteId, 2);
id_type!(WorkerId, 3);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub [u8; 16]);

impl EventId {
    pub fn new() -> Self {
        Self(*Uuid::new_v4().as_bytes())
    }

    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    pub const fn test_id(index: u32) -> Self {
        let mut bytes = [0u8; 16];
        let idx = index.to_le_bytes();
        bytes[0] = idx[0];
        bytes[1] = idx[1];
        bytes[2] = idx[2];
        bytes[3] = idx[3];
        bytes[15] = 4;
        Self(bytes)
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EventId({:02x?})", self.0)
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_test_ids_are_stable() {
        assert_eq!(MachineId::test_id(7), MachineId::test_id(7));
        assert_ne!(MachineId::test_id(7), MachineId::test_id(8));
    }
}
