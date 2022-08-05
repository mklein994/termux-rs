#![cfg(target_os = "android")]

pub mod error;
pub mod termux_battery_status;
pub mod termux_location;

use std::{path::PathBuf, process::Command};

pub use error::Error;

#[derive(Debug, strum::Display, Clone, Copy)]
pub enum CommandKind {
    Location,
    BatteryStatus,
}

#[derive(Debug)]
pub struct TermuxApiCommand {
    inner: Command,
}

impl TermuxApiCommand {
    pub fn arg<S>(&mut self, arg: S) -> &mut Self
    where
        S: AsRef<std::ffi::OsStr>,
    {
        self.inner.arg(arg);
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        self.inner.args(args);
        self
    }

    pub fn get_output(&mut self) -> Result<String, Error> {
        Ok(String::from_utf8(self.inner.output()?.stdout)?)
    }
}

pub trait TermuxApiBuilder {
    const KIND: CommandKind;

    fn new() -> TermuxApiCommand {
        let mut cmd = Command::new(
            PathBuf::from(option_env!("PREFIX").unwrap_or("/data/data/com.termux/files/usr"))
                .join("libexec/termux-api"),
        );
        cmd.arg(Self::KIND.to_string());
        TermuxApiCommand { inner: cmd }
    }
}
