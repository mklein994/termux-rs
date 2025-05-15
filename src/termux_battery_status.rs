use crate::{Error, TermuxApiBuilder};

/// Get the battery status for the device.
///
/// ```no_run
/// use termux_rs::termux_battery_status::TermuxBatteryStatus;
///
/// let status = TermuxBatteryStatus::battery_status().unwrap();
///
/// assert_eq!(r#"{
///   "health": "GOOD",
///   "percentage": 100,
///   "plugged": "PLUGGED_AC",
///   "status": "FULL",
///   "temperature": 31.700000762939453,
///   "current": -65917
/// }
/// "#, status);
/// ```
pub struct TermuxBatteryStatus;

impl TermuxBatteryStatus {
    pub fn battery_status() -> Result<String, Error> {
        Self::new().get_output()
    }
}

impl TermuxApiBuilder for TermuxBatteryStatus {
    const KIND: crate::CommandKind = crate::CommandKind::BatteryStatus;
}
