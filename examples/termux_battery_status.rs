use termux_rs::termux_battery_status::TermuxBatteryStatus;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = TermuxBatteryStatus::battery_status()?;
    print!("{status}");
    Ok(())
}
