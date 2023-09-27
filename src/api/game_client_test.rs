#[cfg(test)]
mod tests {
    use crate::api::protocol::{Move, ProtocolPacket, DataClass, Coordinate, Data};

    

    #[test]
    fn test_new_move() {
        let m: Move = Move {
            from: Coordinate::from(Coordinate { q: 0, r: 0, s: 0 }),
            to: Coordinate::from(Coordinate {q: 0, r: 0, s: 0 }),
        };
        let data: Data = Data::new_move(m);
        assert_eq!(data.class, DataClass::Move);
        assert_eq!(data.color, None);
        assert_eq!(data.state, None);
        assert_eq!(data._move.as_ref().unwrap().from, Coordinate { q: 0, r: 0, s: 0 });
        assert_eq!(data._move.unwrap().to, Coordinate { q: 0, r: 0, s: 0 });
        assert_eq!(data.definition, None);
        assert_eq!(data.scores, None);
        assert_eq!(data.winner, None);
    }

    #[test]
    fn test_join() {
        let xml_str: &str = r#"<join/>"#;
        let packet: ProtocolPacket = ProtocolPacket::parse(xml_str).unwrap();
        match packet {
            ProtocolPacket::Join(_) => {}
            _ => panic!("Expected Join packet"),
        }
    }

    #[test]
    fn test_joined() {
        let xml_str: &str = r#"<joined/>"#;
        let packet: ProtocolPacket = ProtocolPacket::parse(xml_str).unwrap();
        match packet {
            ProtocolPacket::Joined(_) => {}
            _ => panic!("Expected Joined packet"),
        }
    }

    #[test]
    fn test_room() {
        let xml_str: &str = r#"<room roomId="abc123"><data class="moveRequest"/></room>"#;
        let packet: ProtocolPacket = ProtocolPacket::parse(xml_str).unwrap();
        match packet {
            ProtocolPacket::Room(room) => {
                assert_eq!(room.room_id, "abc123");
                assert_eq!(room.data.class, DataClass::MoveRequest);
            }
            _ => panic!("Expected Room packet"),
        }
    }

    #[test]
    fn test_left() {
        let xml_str: &str = r#"<left/>"#;
        let packet: ProtocolPacket = ProtocolPacket::parse(xml_str).unwrap();
        match packet {
            ProtocolPacket::Left(_) => {}
            _ => panic!("Expected Left packet"),
        }
    }

    #[test]
    fn test_join_prepared() {
        let xml_str: &str = r#"<joinPrepared reservationCode="1234" />"#;
        let packet: ProtocolPacket = ProtocolPacket::parse(xml_str).unwrap();
        match packet {
            ProtocolPacket::JoinPrepared(join_prepared) => {
                assert_eq!(join_prepared.reservation_code, "1234");
            }
            _ => panic!("Expected JoinPrepared packet"),
        }
    }

    #[test]
    fn test_message() {
        let xml_str: &str = r#"<errorpacket><message>Invalid move</message></errorpacket>"#;
        let packet: ProtocolPacket = ProtocolPacket::parse(xml_str).unwrap();
        match packet {
            ProtocolPacket::ErrorPacket(error_packet) => {
                assert_eq!(error_packet.message, "Invalid move");
            }
            _ => panic!("Expected ErrorPacket packet"),
        }
    }

    #[test]
    fn test_close() {
        let xml_str: &str = r#"<close/>"#;
        let packet: ProtocolPacket = ProtocolPacket::parse(xml_str).unwrap();
        match packet {
            ProtocolPacket::Close(_) => {}
            _ => panic!("Expected Close packet"),
        }
    }

    #[test]
    fn test_invalid() {
        let xml_str: &str = r#"<invalid/>"#;
        let result: Result<ProtocolPacket, crate::api::game_client::ProtocolError> = ProtocolPacket::parse(xml_str);
        assert!(result.is_err());

    }
}