//! Unified proto module that abstracts over modern vs legacy proto dependencies.
//!
//! Use `crate::proto::*` instead of importing helium_proto, beacon, or helium_crypto directly.
//! Also provides unified http and http_serde re-exports for version compatibility.

// Re-export http crate (modern uses 1.x, legacy uses 0.2.x)
#[cfg(not(feature = "legacy-proto"))]
pub use http;
#[cfg(feature = "legacy-proto")]
pub use http_legacy as http;

// Re-export http_serde crate (modern uses 2.x, legacy uses 1.x)
#[cfg(not(feature = "legacy-proto"))]
pub use http_serde;
#[cfg(feature = "legacy-proto")]
pub use http_serde_legacy as http_serde;

// Re-export the appropriate helium_proto crate
#[cfg(not(feature = "legacy-proto"))]
pub use helium_proto::*;
#[cfg(feature = "legacy-proto")]
pub use helium_proto_legacy::*;

// Re-export beacon crate
#[cfg(not(feature = "legacy-proto"))]
pub use beacon::{self, Beacon, Entropy, Region, RegionParams, BEACON_PAYLOAD_SIZE};
#[cfg(feature = "legacy-proto")]
pub use beacon_legacy::{
    self as beacon, Beacon, Entropy, Region, RegionParams, BEACON_PAYLOAD_SIZE,
};

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

// Re-export tonic transport types (modern uses 0.14, legacy uses 0.10)
pub mod transport {
    #[cfg(not(feature = "legacy-proto"))]
    pub use tonic::transport::{Channel, Endpoint, Uri};
    #[cfg(feature = "legacy-proto")]
    pub use tonic_legacy::transport::{Channel, Endpoint, Uri};
}

// Re-export tonic core types
pub mod tonic {
    #[cfg(not(feature = "legacy-proto"))]
    pub use tonic::*;
    #[cfg(feature = "legacy-proto")]
    pub use tonic_legacy::*;
}

// Convenience re-exports for commonly used service types
pub mod services {
    #[cfg(not(feature = "legacy-proto"))]
    pub use helium_proto::services::*;
    #[cfg(feature = "legacy-proto")]
    pub use helium_proto_legacy::services::*;
}

// Error type re-exports for use in error.rs
// These consolidate all conditional error types in one place
pub mod error {
    // Crypto errors
    #[cfg(not(feature = "legacy-proto"))]
    pub use helium_crypto::Error as CryptoError;
    #[cfg(feature = "legacy-proto")]
    pub use helium_crypto_legacy::Error as CryptoError;

    // Beacon errors
    #[cfg(not(feature = "legacy-proto"))]
    pub use beacon::Error as BeaconError;
    #[cfg(feature = "legacy-proto")]
    pub use beacon_legacy::Error as BeaconError;

    // Prost encode/decode errors (re-exported by helium_proto)
    #[cfg(not(feature = "legacy-proto"))]
    pub use helium_proto::{DecodeError as ProstDecodeError, EncodeError as ProstEncodeError};
    #[cfg(feature = "legacy-proto")]
    pub use helium_proto_legacy::{
        DecodeError as ProstDecodeError, EncodeError as ProstEncodeError,
    };

    // Tonic/service errors - these differ between modern and legacy
    #[cfg(feature = "legacy-proto")]
    pub use helium_proto_legacy::services::Error as ServiceConnectError;
    #[cfg(not(feature = "legacy-proto"))]
    pub use tonic::transport::Error as ServiceConnectError;

    #[cfg(not(feature = "legacy-proto"))]
    pub use tonic::Status as RpcStatus;
    #[cfg(feature = "legacy-proto")]
    pub use tonic_legacy::Status as RpcStatus;
}
