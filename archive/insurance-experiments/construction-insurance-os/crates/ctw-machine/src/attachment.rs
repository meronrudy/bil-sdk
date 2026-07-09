//! Attachments and work tools.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type of attachment currently fitted.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AttachmentType {
    StandardBucket,
    DitchCleaningBucket,
    Breaker,
    Grapple,
    Tiltrotator,
    Auger,
    Compactor,
    Ripper,
    QuickCoupler,
    Fork,
    None,
}
