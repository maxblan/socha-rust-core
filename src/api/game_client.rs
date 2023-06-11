use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

use yaserde::de::from_str;
use yaserde::ser::to_string;

use crate::api::protocol::*;
use crate::api::xml_buffer::XmlBuffer;

pub trait ClientInterface {
    fn on_state(&mut self, state: Room);
    fn on_move_request(&mut self) -> Move;
    fn on_error_packet(&mut self, error_packet: ErrorPacket);
    fn on_erroneous_move(&mut self, error: Room);
    fn on_joined(&mut self, joined: Joined);
    fn on_welcome(&mut self, welcome_message: Room);
    fn on_left(&mut self, left: Left);
    fn on_result(&mut self, result: Room);
}

pub struct GameClient {
    host: String,
    port: u16,
    reservation_code: Option<String>,
    room_id: Option<String>,
    client_interface: Box<dyn ClientInterface>,
    stream: TcpStream,
    connected: bool,
}

impl GameClient {
    pub fn new(
        host: String,
        port: u16,
        reservation_code: Option<String>,
        room_id: Option<String>,
        client_interface: Box<dyn ClientInterface>,
    ) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect((host.as_str(), port))?;
        let connected = false;
        Ok(GameClient {
            host,
            port,
            reservation_code,
            room_id,
            client_interface,
            stream,
            connected,
        })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        self.stream = TcpStream::connect((self.host.as_str(), self.port))?;
        let mut buffer = XmlBuffer::new();

        self.handle_join();
        self.connected = true;
        loop {
            if self.connected == false {
                break;
            }
            let mut buf = [0; 16129];
            let bytes_read = self.stream.read(&mut buf)?;
            buffer.push(&buf[0..bytes_read]);

            while let Some(xml_bytes) = buffer.pop_xml() {
                let xml_str = str::from_utf8(&xml_bytes).unwrap().to_string();
                self.handle_protocol(xml_str).expect("Exception: Could not handle protocol");
            }
        }

        Ok(())
    }

    fn handle_protocol(&mut self, xml_str: String) -> Result<(), std::io::Error> {
        println!("Received: \n{}", xml_str);

        let packet = match ProtocolPacket::parse(xml_str.as_str()) {
            Ok(packet) => packet,
            Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::Other, error.to_string())),
        };

        match packet {
            ProtocolPacket::Room(room) => {
                match room.data.class {
                    DataClass::MoveRequest => {
                        let _move = self.client_interface.on_move_request();
                        let data = Data::new_move(_move);
                        self.send_room(room.room_id, data)?;
                    }
                    DataClass::Memento => self.client_interface.on_state(room),
                    DataClass::Result => self.client_interface.on_result(room),
                    DataClass::WelcomeMessage => self.client_interface.on_welcome(room),
                    DataClass::Move => {}
                    DataClass::Error => self.client_interface.on_erroneous_move(room)
                }
            }
            ProtocolPacket::Joined(joined) => {
                self.client_interface.on_joined(joined);
            }
            ProtocolPacket::Left(left) => {
                self.client_interface.on_left(left);
                self.connected = false;
            }
            ProtocolPacket::ErrorPacket(error) => {
                self.client_interface.on_error_packet(error)
            }
            _ => {}
        }

        Ok(())
    }


    fn send(&mut self, buf: &[u8]) -> std::io::Result<()> {
        println!("Sending: \n{}", str::from_utf8(&buf).unwrap());
        self.stream.write_all(&buf)?;
        Ok(())
    }

    fn send_room(&mut self, room_id: String, data: Data) -> Result<(), std::io::Error> {
        let room = Room { data, room_id };
        let substring_to_remove = r#"<?xml version="1.0" encoding="utf-8"?>"#;
        let serialized_item = to_string(&room).unwrap().replace(substring_to_remove, "");
        self.send(serialized_item.as_ref())
    }

    fn handle_join(&mut self) {
        if self.reservation_code.is_some() {
            self.join_reservation(self.reservation_code.clone().unwrap()).expect("Exception: Could not join with reservation code");
        } else if self.room_id.is_some() {
            self.join_room(self.room_id.clone().unwrap()).expect("Exception: Could not join with room id");
        } else {
            self.join().expect("Exception: Could not join");
        }
    }
    fn join(&mut self) -> Result<(), std::io::Error> {
        let serialized_item = "<protocol><join/>".to_string();
        self.send(serialized_item.as_ref())
    }

    fn join_reservation(&mut self, reservation_code: String) -> Result<(), std::io::Error> {
        let join = JoinPrepared { reservation_code };
        let serialized_item = to_string(&join).unwrap();
        self.send(serialized_item.as_ref())
    }

    fn join_room(&mut self, _room_id: String) -> Result<(), std::io::Error> {
        let join = Join {};
        let serialized_item = to_string(&join).unwrap();
        self.send(serialized_item.as_ref())
    }
}

impl Data {
    fn new_move(m: Move) -> Self {
        Self {
            class: DataClass::Move,
            color: None,
            state: None,
            _move: Some(m),
            definition: None,
            scores: None,
            winner: None,
        }
    }
}

impl ProtocolPacket {
    pub fn parse(xml_str: &str) -> Result<ProtocolPacket, ProtocolError> {
        match xml_str {
            s if s.contains("joined") => {
                let joined: Joined = from_str(&xml_str)
                    .map_err(|_| ProtocolError::ParseError)?;
                Ok(ProtocolPacket::Joined(joined))
            }
            s if s.contains("join") => {
                let join: Join = from_str(&xml_str)
                    .map_err(|_| ProtocolError::ParseError)?;
                Ok(ProtocolPacket::Join(join))
            }
            s if s.contains("room") => {
                let room: Room = from_str(&xml_str)
                    .map_err(|_| ProtocolError::ParseError)?;
                Ok(ProtocolPacket::Room(room))
            }
            s if s.contains("left") => {
                let left: Left = from_str(&xml_str)
                    .map_err(|_| ProtocolError::ParseError)?;
                Ok(ProtocolPacket::Left(left))
            }
            s if s.contains("joinPrepared") => {
                let join_prepared: JoinPrepared = from_str(&xml_str)
                    .map_err(|_| ProtocolError::ParseError)?;
                Ok(ProtocolPacket::JoinPrepared(join_prepared))
            }
            s if s.contains("errorpacket") => {
                let error_packet: ErrorPacket = from_str(&xml_str)
                    .map_err(|_| ProtocolError::ParseError)?;
                Ok(ProtocolPacket::ErrorPacket(error_packet))
            }
            s if s.contains("close") => {
                let close: Close = from_str(&xml_str)
                    .map_err(|_| ProtocolError::ParseError)?;
                Ok(ProtocolPacket::Close(close))
            }
            _ => Err(ProtocolError::InvalidPacket(xml_str.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    ParseError,
    InvalidPacket(String),
}

impl Error for ProtocolError {}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::ParseError => write!(f, "Error parsing protocol packet"),
            ProtocolError::InvalidPacket(msg) => write!(f, "Invalid protocol packet: {}", msg),
        }
    }
}


