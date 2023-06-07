use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct PacketError {
    description: String,
}

impl From<&str> for PacketError {
    fn from(value: &str) -> Self {
        Self {
            description: value.to_string()
        }
    }
}

impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.description)?;
        Ok(())
    }
}

impl Error for PacketError {}
