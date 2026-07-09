//! Raw ingress frames from hardware.

use ctw_core::{SensorId, MonotonicMicros};

/// A raw data frame from a sensor or bus before interpretation.
#[derive(Clone, Debug)]
pub enum RawFrame {
    /// CAN bus frame (J1939 or proprietary).
    Can {
        sensor_id: SensorId,
        timestamp: MonotonicMicros,
        arbitration_id: u32,
        data: [u8; 8],
        dlc: u8,
    },
    /// Inertial measurement unit packet.
    Imu {
        sensor_id: SensorId,
        timestamp: MonotonicMicros,
        /// Acceleration XYZ in m/s² (raw, before calibration).
        accel_raw: [f32; 3],
        /// Angular rate XYZ in rad/s (raw).
        gyro_raw: [f32; 3],
    },
    /// GNSS/RTK position fix.
    Gnss {
        sensor_id: SensorId,
        timestamp: MonotonicMicros,
        latitude_deg: f64,
        longitude_deg: f64,
        altitude_m: f32,
        fix_quality: GnssFixQuality,
        hdop: f32,
    },
    /// Worker/asset tag position from UWB/RTLS.
    TagPosition {
        sensor_id: SensorId,
        timestamp: MonotonicMicros,
        tag_id: u64,
        x_m: f32,
        y_m: f32,
        z_m: f32,
        accuracy_m: f32,
    },
    /// Object detection from vision system.
    VisionDetection {
        sensor_id: SensorId,
        timestamp: MonotonicMicros,
        detections: alloc::vec::Vec<Detection>,
    },
}

/// GNSS fix quality level.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GnssFixQuality {
    NoFix,
    SinglePoint,
    Dgps,
    RtkFloat,
    RtkFixed,
}

/// A single object detection from a vision system.
#[derive(Clone, Debug)]
pub struct Detection {
    pub class: DetectionClass,
    pub confidence: f32,
    pub distance_m: Option<f32>,
    pub bearing_rad: Option<f32>,
}

/// Object classes relevant to construction safety.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DetectionClass {
    Person,
    Vehicle,
    HeavyEquipment,
    Cone,
    Barricade,
    TrenchEdge,
    Overhead,
    Unknown,
}

extern crate alloc;
