use std::collections::HashMap;
use std;
use serde;
use serde_json;
use {CharacterId, Timestamp, WorldId};

#[derive(Deserialize, PartialEq, Debug)]
#[serde(tag = "event_name")]
pub enum Event {
    PlayerLogin {
        #[serde(deserialize_with = "deserialize_fromstr")] character_id: CharacterId,
        #[serde(deserialize_with = "deserialize_fromstr")] timestamp: Timestamp,
        #[serde(deserialize_with = "deserialize_fromstr")] world_id: WorldId,
    },
    PlayerLogout {
        #[serde(deserialize_with = "deserialize_fromstr")] character_id: CharacterId,
        #[serde(deserialize_with = "deserialize_fromstr")] timestamp: Timestamp,
        #[serde(deserialize_with = "deserialize_fromstr")] world_id: WorldId,
    },
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub character_count: u64,
    pub event_names: Vec<String>,
    pub logical_and_characters_with_worlds: bool,
    pub worlds: Vec<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Message {
    ConnectionStateChanged {
        #[serde(deserialize_with = "deserialize_fromstr")] connected: bool,
    },
    Heartbeat {
        online: HashMap<String, String>,
    },
    ServiceMessage {
        payload: Event,
    },
    ServiceStateChanged {
        #[serde(deserialize_with = "deserialize_fromstr")] online: bool,
        detail: String,
    },
    Subscription {
        subscription: Subscription,
    },
}

fn deserialize_fromstr<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
// TODO: Replace `assert_eq!(.., json)` with https://docs.serde.rs/serde_test/
mod tests {
    use super::*;

    #[test]
    fn service_state_changed() {
        let input = r#"{
            "detail": "EventServerEndpoint_Cobalt_13",
            "online": "true",
            "service": "event",
            "type": "serviceStateChanged"
        }"#;
        let deserialized: Message = serde_json::from_str(input).unwrap();

        let expected = Message::ServiceStateChanged {
            online: true,
            detail: "EventServerEndpoint_Cobalt_13".to_string(),
        };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn connection_state_changed() {
        let input = r#"{
            "connected": "true",
            "service": "push",
            "type": "connectionStateChanged"
        }"#;
        let deserialized: Message = serde_json::from_str(input).unwrap();

        let expected = Message::ConnectionStateChanged { connected: true };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn player_login() {
        /*
         * NOTE: For `timestamp` see https://serde.rs/custom-date-format.html and http://census.daybreakgames.com/#dates-timestamps
         */
        let input = r#"{
            "payload": {
            "character_id": "5428602376718262177",
                "event_name": "PlayerLogin",
                "timestamp": "1513785744",
                "world_id": "1"
            },
            "service": "event",
            "type": "serviceMessage"
        }"#;
        let deserialized: Message = serde_json::from_str(input).unwrap();

        let expected = Message::ServiceMessage {
            payload: Event::PlayerLogin {
                character_id: 5428602376718262177,
                timestamp: 1513785744,
                world_id: 1,
            },
        };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn heartbeat() {
        let input = r#"{
            "online": {
                "EventServerEndpoint_Briggs_25": "true",
                "EventServerEndpoint_Cobalt_13": "true",
                "EventServerEndpoint_Connery_1": "true",
                "EventServerEndpoint_Emerald_17": "true",
                "EventServerEndpoint_Jaeger_19": "true",
                "EventServerEndpoint_Miller_10": "true"
            },
            "service": "event",
            "type": "heartbeat"
        }"#;
        let deserialized: Message = serde_json::from_str(input).unwrap();

        let mut online: HashMap<String, String> = HashMap::new();
        online.insert(
            "EventServerEndpoint_Briggs_25".to_string(),
            "true".to_string(),
        );
        online.insert(
            "EventServerEndpoint_Cobalt_13".to_string(),
            "true".to_string(),
        );
        online.insert(
            "EventServerEndpoint_Connery_1".to_string(),
            "true".to_string(),
        );
        online.insert(
            "EventServerEndpoint_Emerald_17".to_string(),
            "true".to_string(),
        );
        online.insert(
            "EventServerEndpoint_Jaeger_19".to_string(),
            "true".to_string(),
        );
        online.insert(
            "EventServerEndpoint_Miller_10".to_string(),
            "true".to_string(),
        );
        let expected = Message::Heartbeat { online: online };

        assert_eq!(deserialized, expected);
    }

    #[test]
    fn subscription() {
        let input = r#"{
            "subscription": {
                "characterCount": 0,
                "eventNames": [
                    "PlayerLogin"
                ],
                "logicalAndCharactersWithWorlds": false,
                "worlds": [
                    "1"
                ]
            }
        }"#;
        let deserialized: Message = serde_json::from_str(input).unwrap();

        let expected = Message::Subscription {
            subscription: Subscription {
                character_count: 0,
                event_names: vec!["PlayerLogin".to_string()],
                logical_and_characters_with_worlds: false,
                worlds: vec!["1".to_string()],
            },
        };

        assert_eq!(deserialized, expected);
    }
}
