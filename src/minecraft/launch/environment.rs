use serde::Deserialize;
use std::env::consts;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum OS {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "osx")]
    MacOS,
    #[serde(rename = "linux")]
    Linux,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
    X86,
    X64,
    Arm64,
}

#[derive(Debug, PartialEq)]
pub struct Environment {
    pub os: OS,
    pub arch: Arch,
}

impl TryFrom<&str> for OS {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "windows" => Ok(OS::Windows),
            "macos" => Ok(OS::MacOS),
            "linux" => Ok(OS::Linux),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Arch {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x86" | "i686" => Ok(Arch::X86),
            "x86_64" => Ok(Arch::X64),
            "aarch64" => Ok(Arch::Arm64),
            _ => Err(()),
        }
    }
}

impl Environment {
    pub fn detect() -> Self {
        let os = OS::try_from(consts::OS).expect("unsupported operating system");

        let arch = Arch::try_from(consts::ARCH).expect("unsupported architecture");

        Self { os, arch }
    }
}
