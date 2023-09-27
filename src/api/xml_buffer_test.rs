#[cfg(test)]
mod tests {
    use crate::api::xml_buffer::XmlBuffer;

    #[test]
    fn test_push_and_pop_xml() {
        let mut buffer: XmlBuffer = XmlBuffer::new();

        // Push some bytes onto the buffer
        buffer.push(b"<room><name>Test Room</name></room>");
        buffer.push(b"<errorpacket><message>Invalid username</message></errorpacket>");
        buffer.push(b"<room><name>Another Room</name></room>");

        // Pop the first XML element from the buffer
        let xml_bytes: Vec<u8> = buffer.pop_xml().unwrap();
        assert_eq!(xml_bytes, b"<room><name>Test Room</name></room>");

        // Pop the second XML element from the buffer
        let xml_bytes: Vec<u8> = buffer.pop_xml().unwrap();
        assert_eq!(xml_bytes, b"<errorpacket><message>Invalid username</message></errorpacket>");

        // Push some more bytes onto the buffer
        buffer.push(b"<room><name>Third Room</name></room>");

        // Pop the third XML element from the buffer
        let xml_bytes: Vec<u8> = buffer.pop_xml().unwrap();
        assert_eq!(xml_bytes, b"<room><name>Another Room</name></room>");
    }
}