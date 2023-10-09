#[cfg(test)]
mod tests {
    use crate::api::protocol::{Move, Join, DataClass, Coordinate, Data, Room, CubeDirection, FieldArray, Water, Island, Sandbank, Passenger, Goal, State, Ship, Segment, Board, Team};

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

    #[test]
    fn test_field_array_struct() {
        let xml: &str = r#"
        <field-array>
        <water/>
        <island/>
        <sandbank/>
        <passenger direction="RIGHT" passenger="1" />
        <goal/>
      </field-array>
    "#; 

    let field_array: FieldArray = yaserde::de::from_str(xml).unwrap();

        assert_eq!(
            field_array, 
            FieldArray { water: vec![Water], island: vec![Island], sandbank: vec![Sandbank], passenger: vec![Passenger { direction: CubeDirection::Right, passenger: 1 }], goal: vec![Goal] }
        );
    }

    #[test]
    fn test_state_struct() {
        let xml: &str = r#"
        <state class="state" startTeam="ONE" turn="0" currentTeam="ONE">
        <board nextDirection="DOWN_RIGHT">
          <segment direction="RIGHT">
            <center q="0" r="0" s="0"/>
            <field-array>
              <water/>
              <water/>
              <water/>
              <water/>
              <water/>
            </field-array>
          </segment>
        </board>
        <ship team="ONE" direction="RIGHT" speed="1" coal="6" passengers="0" freeTurns="1" points="0">
          <position q="-1" r="-1" s="2"/>
        </ship>
        <ship team="TWO" direction="RIGHT" speed="1" coal="6" passengers="0" freeTurns="1" points="0">
          <position q="-2" r="1" s="1"/>
        </ship>
      </state>
    "#; 

    let field_array: State = yaserde::de::from_str(xml).unwrap();

        assert_eq!(
            field_array, 
            State { 
                class: "state".to_string(), 
                start_team: Team::One, 
                turn: 0, 
                current_team: Team::One, 
                board: Board { 
                    next_direction: CubeDirection::DownRight, 
                    segment: vec![Segment { 
                        direction: CubeDirection::Right, 
                        center: Coordinate { q: 0, r: 0, s: 0 }, 
                        field_array: vec![
                            FieldArray { 
                                water: vec![Water, Water, Water, Water, Water], 
                                island: vec![], 
                                sandbank: vec![], 
                                passenger: vec![], 
                                goal: vec![] }
                        ] }] }, 
                        ship: vec![
                            Ship { 
                            team: Team::One, 
                            direction: CubeDirection::Right, 
                            speed: 1, 
                            coal: 6, 
                            passengers: 0, 
                            free_turns: 1, 
                            points: 0, 
                            position: Coordinate { q: -1, r: -1, s: 2 } }, 
                            Ship { 
                                team: Team::Two,
                                direction: CubeDirection::Right, 
                                speed: 1, 
                                coal: 6, 
                                passengers: 0, 
                                free_turns: 1, 
                                points: 0, 
                                position: Coordinate { q: -2, r: 1, s: 1 } }] }
        );
    }
}