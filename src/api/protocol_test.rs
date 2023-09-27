#[cfg(test)]
mod tests {
    use crate::api::protocol::{Move, Join, DataClass, Coordinate, Data, Room};

    #[test]
    fn test_join_deserialization() {
        let xml = r#"
            <join />
        "#;
        let join: Join = yaserde::de::from_str(xml).unwrap();
        assert_eq!(join, Join {});
    }

    #[test]
    fn test_room_deserialization() {
        let xml: &str = r#"
            <room roomId="room-1">
                <data class="move">
                <from q="0" r="0" s="0" />
                <to q="1" r="0" s="-1" />
                </data>
            </room>
        "#;

        let room: Room = yaserde::de::from_str(xml).unwrap();
        assert_eq!(
            room,
            Room {
                room_id: "room-1".to_string(),
                data: Data {
                    class: DataClass::Move,
                    color: None,
                    state: None,
                    _move: Some(Move {
                        from: Coordinate { q: 0, r: 0, s: 0 },
                        to: Coordinate { q: 1, r:0, s: -1 },
                    }),
                    definition: None,
                    scores: None,
                    winner: None,
                },
            }
        );
    }

    #[test]
    fn test_coordinate_deserialization() {
        let xml = r#"
            <from q="1" r="2" s="3" />
        "#;
        let coordinate: Coordinate = yaserde::de::from_str(xml).unwrap();
        assert_eq!(
            coordinate,
            Coordinate { q: 1, r: 2, s: 3 }
        );
    }
}