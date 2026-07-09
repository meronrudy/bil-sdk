//! The engine crate ties everything together.
//! It provides the complete pipeline: telemetry → risk → pricing → filing.

pub use ctw_core;
pub use ctw_geo;
pub use ctw_machine;
pub use ctw_ingest;
pub use ctw_risk;
pub use ctw_exposure;
pub use ctw_context;
pub use actuarial_core;
pub use actuarial_model;
pub use actuarial_pricing;
pub use actuarial_reserving;
pub use actuarial_explain;
pub use actuarial_governance;

/// Version of the engine.
pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
