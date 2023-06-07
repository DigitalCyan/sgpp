use crate::error::PacketError;

pub const PACKET_MIN_SIZE: u32 = 9;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    size: u32,
    kind: u32,
    body: Vec<u8>,
}

impl Packet {
    pub fn new(kind: u32, body: &Vec<u8>) -> Self {
        Self {
            size: PACKET_MIN_SIZE + body.len() as u32,
            kind,
            body: body.clone(),
        }
    }
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.append(&mut self.size.to_be_bytes().iter().map(|b| *b).collect::<Vec<u8>>());
        buf.append(&mut self.kind.to_be_bytes().iter().map(|b| *b).collect::<Vec<u8>>());
        buf.append(&mut self.body.clone());
        buf.push('\0' as u8);

        buf
    }
}

impl TryFrom<Vec<u8>> for Packet {
    type Error = PacketError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() < PACKET_MIN_SIZE as usize {
            return Err(PacketError::from("Illegal packet size"));
        }

        let mut u64_buf = [0u8; 4];

        for (i, byte) in value.iter().take(4).enumerate() {
            u64_buf[i] = *byte;
        }

        let size = u32::from_be_bytes(u64_buf.clone());

        for (i, byte) in value.iter().skip(4).take(4).enumerate() {
            u64_buf[i] = *byte;
        }

        let kind = u32::from_be_bytes(u64_buf);

        let body = value.into_iter().skip(8).take((size - PACKET_MIN_SIZE) as usize).collect();

        Ok(Packet::new(kind, &body))
    }
}
