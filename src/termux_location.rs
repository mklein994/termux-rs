use super::TermuxApiBuilder;

/// Get coordinates and other location info.
///
/// ```no_run
/// use termux_rs::termux_location::{Provider, Request, TermuxLocation};
///
/// let location = TermuxLocation::termux_location(Provider::Gps, Request::Once).unwrap();
/// assert_eq!(
///     r#"{
///   "latitude": 52.96317238,
///   "longitude": -108.74197643,
///   "altitude": 546.208984375,
///   "accuracy": 20.12640953063965,
///   "vertical_accuracy": 10.85973072052002,
///   "bearing": 113.5,
///   "speed": 30.549999237060547,
///   "elapsedMs": 56,
///   "provider": "gps"
/// }
/// "#,
///     location
/// );
/// ```
pub struct TermuxLocation;

impl TermuxLocation {
    pub fn termux_location(provider: Provider, request: Request) -> Result<String, super::Error> {
        Self::new()
            .arg("--es")
            .arg(provider.as_ref())
            .arg("--es")
            .arg(request.as_ref())
            .get_output()
    }
}

impl TermuxApiBuilder for TermuxLocation {
    const KIND: crate::CommandKind = crate::CommandKind::Location;
}

#[derive(Debug, Default, strum::AsRefStr, Clone, Copy)]
#[cfg_attr(feature = "example", derive(clap::ValueEnum))]
pub enum Provider {
    #[default]
    Gps,
    Network,
    Passive,
}

#[derive(Debug, Default, strum::AsRefStr, Clone, Copy)]
#[cfg_attr(feature = "example", derive(clap::ValueEnum))]
pub enum Request {
    #[default]
    Once,
    Last,
    Updates,
}
