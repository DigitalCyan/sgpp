pub mod packet;
pub mod error;

#[cfg(test)]
mod tests {
    use crate::packet::Packet;

    #[test]
    fn new_packet() {
        Packet::new(0, &Vec::new());
    }

    #[test]
    fn packet_to_vec() {
        let packet = Packet::new(0, &vec![65]);

        let buf: Vec<u8> = packet.into();

        assert_eq!(buf, vec![0, 0, 0, 10, 0, 0, 0, 0, 65, 10]);
    }

    #[test]
    fn ser_and_deser() {
        let packet = Packet::new(0, &vec![65]);
        let buf: Vec<u8> = packet.clone().into();

        let Ok(deser_packet) = Packet::try_from(buf) else {
            panic!("Failed to deserialize packet");
        };

        assert_eq!(packet, deser_packet);
    }
}
