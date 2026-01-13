//! Unified proto module that abstracts over modern vs legacy proto dependencies.
//!
//! Use `crate::proto::*` instead of importing helium_proto, beacon, or helium_crypto directly.

// Re-export the appropriate helium_proto crate
#[cfg(not(feature = "legacy-proto"))]
pub use helium_proto::*;
#[cfg(feature = "legacy-proto")]
pub use helium_proto_legacy::*;

// Re-export beacon crate
#[cfg(not(feature = "legacy-proto"))]
pub use beacon::{self, Beacon, Entropy, Region, RegionParams, BEACON_PAYLOAD_SIZE};
#[cfg(feature = "legacy-proto")]
pub use beacon_legacy::{self as beacon, Beacon, Entropy, Region, RegionParams, BEACON_PAYLOAD_SIZE};

// Re-export helium_crypto crate
pub mod crypto {
    #[cfg(not(feature = "legacy-proto"))]
    pub use helium_crypto::*;
    #[cfg(feature = "legacy-proto")]
    pub use helium_crypto_legacy::*;

    // Re-export signature for legacy builds since helium_crypto_legacy doesn't include it
    #[cfg(feature = "legacy-proto")]
    pub use signature;
}

// Re-export tonic types from the services module (both use tonic 0.10)
pub mod transport {
    #[cfg(not(feature = "legacy-proto"))]
    pub use helium_proto::services::{Channel, Endpoint, Uri};
    #[cfg(feature = "legacy-proto")]
    pub use helium_proto_legacy::services::{Channel, Endpoint, Uri};
}

// Convenience re-exports for commonly used service types
pub mod services {
    #[cfg(not(feature = "legacy-proto"))]
    pub use helium_proto::services::*;
    #[cfg(feature = "legacy-proto")]
    pub use helium_proto_legacy::services::*;
}
