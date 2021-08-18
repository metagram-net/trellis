use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_round_trip_json() {
    let settings = config::Config {
        secrets: config::Secrets {
            owm_api_key: Some("TEST_OWM_API_KEY".to_owned()),
        },
        tiles: vec![
            config::Tile {
                id: uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
                width: None,
                height: None,
                data: config::Data::Note {
                    text: "".to_owned(),
                },
            },
            config::Tile {
                id: uuid::Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
                width: Some(3),
                height: Some(4),
                data: config::Data::Weather {
                    location_id: "1234567".to_owned(),
                },
            },
            config::Tile {
                id: uuid::Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap(),
                width: None,
                height: None,
                data: config::Data::Clock,
            },
        ],
    };
    let expected = r#"{"secrets":{"owm_api_key":"TEST_OWM_API_KEY"},"tiles":[{"id":"00000000-0000-0000-0000-000000000000","data":{"type":"Note","text":""}},{"id":"11111111-1111-1111-1111-111111111111","width":3,"height":4,"data":{"type":"Weather","location_id":"1234567"}},{"id":"33333333-3333-3333-3333-333333333333","data":{"type":"Clock"}}]}"#;

    let serialized = serde_json::to_string(&settings).unwrap();
    assert_eq!(serialized, expected);

    let deserialized: config::Config = serde_json::from_str(&expected).unwrap();
    assert_eq!(deserialized, settings);
}
