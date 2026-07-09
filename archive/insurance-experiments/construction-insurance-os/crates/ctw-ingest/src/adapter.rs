//! Adapter trait: how OEM-specific data enters the canonical pipeline.

use ctw_core::MonotonicMicros;
use crate::frame::RawFrame;
use crate::observation::Observation;

/// An adapter converts raw vendor-specific frames into canonical observations.
///
/// Each OEM/sensor vendor implements this trait. The adapter is the only
/// place that understands vendor-specific byte layouts, DBC files,
/// and protocol quirks.
pub trait IngestAdapter {
    /// The error type for this adapter.
    type Error: core::fmt::Debug;

    /// Process a raw frame and emit zero or more canonical observations.
    ///
    /// Returns the number of observations emitted via the sink.
    fn ingest(
        &mut self,
        timestamp: MonotonicMicros,
        frame: &RawFrame,
        sink: &mut dyn ObservationSink,
    ) -> Result<usize, Self::Error>;

    /// Human-readable name of this adapter.
    fn name(&self) -> &str;

    /// Reset internal state (e.g., between test runs or sessions).
    fn reset(&mut self);
}

/// Sink for observations emitted by adapters.
pub trait ObservationSink {
    fn emit(&mut self, observation: Observation);
}
