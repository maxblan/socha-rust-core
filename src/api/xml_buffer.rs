use regex::bytes::Regex;

pub struct XmlBuffer {
    buffer: Vec<u8>,
}

impl XmlBuffer {
    pub fn new() -> Self {
        XmlBuffer { buffer: Vec::new() }
    }

    pub fn push(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn pop_xml(&mut self) -> Option<Vec<u8>> {
        let regex_str: &str = r#"<((room[\s\S]+?</room>)|errorpacket[\s\S]+?</errorpacket>|.*?/>)"#;
        let regex: Regex = Regex::new(regex_str).unwrap();

        match regex.find(&self.buffer) {
            Some(mat) => {
                let xml_bytes: Vec<u8> = self.buffer[mat.start()..mat.end()].to_vec();
                self.buffer.drain(0..mat.end());
                Some(xml_bytes)
            }
            None => None,
        }
    }
}
