extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub type Id = u64;
pub type CharacterId = Id;
pub type WorldId = Id;
pub type ExperienceId = Id;

pub type Timestamp = u64;

// TODO: s:example
pub type ServiceId = String;

pub enum Environment {
    Pc,
    Ps4Us,
    Ps4Eu,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Service {
    Event,
    Push,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum CharacterSubscription {
    #[serde(serialize_with = "serialize_all_subscription")] All,
    #[serde(serialize_with = "serialize_ids_subscription")] Ids(Vec<CharacterId>),
}

#[repr(u64)]
pub enum WorldIds {
    Jaeger = 19,
    Briggs = 25,
    Miller = 10,
    Cobalt = 13,
    Connery = 1,
    Emerald = 17,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum WorldSubscription {
    #[serde(serialize_with = "serialize_all_subscription")] All,
    #[serde(serialize_with = "serialize_ids_subscription")] Ids(Vec<WorldId>),
}

#[derive(Serialize, PartialEq, Eq, Hash)]
pub enum EventNames {
    AchievementEarned,
    BattleRankUp,
    Death,
    ItemAdded,
    SkillAdded,
    VehicleDestroy,
    GainExperience,
    GainExperienceId(ExperienceId),
    PlayerFacilityCapture,
    PlayerFacilityDefend,
    ContinentLock,
    ContinentUnlock,
    FacilityControl,
    MetagameEvent,
    PlayerLogin,
    PlayerLogout,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum EventSubscription {
    #[serde(serialize_with = "serialize_all_subscription")] All,
    Ids(Vec<EventNames>),
}

#[derive(Serialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum Action {
    Echo {
        payload: serde_json::Value,
        service: Service,
    },
    #[serde(rename_all = "camelCase")]
    Subscribe {
        #[serde(skip_serializing_if = "Option::is_none")] event_names: Option<EventSubscription>,
        #[serde(skip_serializing_if = "Option::is_none")] characters: Option<CharacterSubscription>,
        #[serde(skip_serializing_if = "Option::is_none")]
        logical_and_characters_with_worlds: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")] worlds: Option<WorldSubscription>,
        service: Service,
    },
    #[serde(rename_all = "camelCase")]
    ClearSubscribe {
        #[serde(skip_serializing_if = "Option::is_none",
                serialize_with = "serialize_optional_bool")]
        all: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")] event_names: Option<EventSubscription>,
        #[serde(skip_serializing_if = "Option::is_none")] characters: Option<CharacterSubscription>,
        #[serde(skip_serializing_if = "Option::is_none")] worlds: Option<WorldSubscription>,
        service: Service,
    },
    RecentCharacterIds {
        service: Service,
    },
    RecentCharacterIdsCount {
        service: Service,
    },
}

fn serialize_optional_bool<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match *value {
        None => serializer.serialize_none(),
        Some(value) => match value {
            true => serializer.serialize_str("true"),
            false => serializer.serialize_str("false"),
        },
    }
}

fn serialize_all_subscription<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;

    json!(["all"]).serialize(serializer)
}

fn serialize_ids_subscription<S>(value: &Vec<Id>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut ids = Vec::with_capacity(value.len());
    for id in value.iter() {
        ids.push(id.to_string());
    }

    serializer.collect_seq(ids.iter())
}

#[cfg(test)]
// TODO: Replace `assert_eq!(.., json!())` with https://docs.serde.rs/serde_test/
mod tests {
    use super::*;

    #[test]
    fn serialize_echo_action() {
        let input = Action::Echo {
            payload: json!({
                "test": "test"
            }),
            service: Service::Event,
        };
        let v = serde_json::to_value(input).unwrap();

        let expected = json!({
            "service": "event",
            "action": "echo",
            "payload": {
                "test": "test"
            }
        });

        assert_eq!(v, expected);
    }

    #[test]
    fn serialize_subscribe_character_death_action() {
        let input = Action::Subscribe {
            event_names: Some(EventSubscription::Ids(vec![EventNames::Death])),
            characters: Some(CharacterSubscription::Ids(vec![5428010618015189713])),
            logical_and_characters_with_worlds: None,
            worlds: None,
            service: Service::Event,
        };
        let v = serde_json::to_value(input).unwrap();

        let expected = json!({
            "service": "event",
            "action": "subscribe",
            "eventNames": [ "Death" ],
            "characters": [ "5428010618015189713" ]
        });

        assert_eq!(v, expected);
    }

    #[test]
    fn serialize_subscribe_world_event_action() {
        let input = Action::Subscribe {
            event_names: Some(EventSubscription::Ids(vec![EventNames::PlayerLogin])),
            characters: None,
            logical_and_characters_with_worlds: None,
            worlds: Some(WorldSubscription::Ids(vec![WorldIds::Connery as WorldId])),
            service: Service::Event,
        };
        let v = serde_json::to_value(input).unwrap();

        let expected = json!({
            "service": "event",
            "action": "subscribe",
            "eventNames": [ "PlayerLogin" ],
            "worlds": [ "1" ]
        });

        assert_eq!(v, expected);
    }

    #[test]
    fn serialize_subscribe_all_action() {
        let input = Action::Subscribe {
            event_names: Some(EventSubscription::All),
            characters: Some(CharacterSubscription::All),
            logical_and_characters_with_worlds: None,
            worlds: Some(WorldSubscription::All),
            service: Service::Event,
        };
        let v = serde_json::to_value(input).unwrap();

        let expected = json!({
            "service": "event",
            "action": "subscribe",
            "eventNames": [ "all" ],
            "characters": [ "all" ],
            "worlds": [ "all" ]
        });

        assert_eq!(v, expected);
    }

    #[test]
    fn serialize_subscribe_action() {
        let input = Action::Subscribe {
            event_names: Some(EventSubscription::Ids(vec![
                EventNames::PlayerLogin,
                EventNames::MetagameEvent,
                EventNames::BattleRankUp,
                EventNames::FacilityControl,
                EventNames::ItemAdded,
                EventNames::VehicleDestroy,
                EventNames::PlayerFacilityCapture,
                EventNames::PlayerFacilityDefend,
                EventNames::SkillAdded,
                EventNames::GainExperience,
                EventNames::Death,
                EventNames::PlayerLogout,
            ])),
            characters: Some(CharacterSubscription::All),
            logical_and_characters_with_worlds: Some(true),
            worlds: Some(WorldSubscription::Ids(vec![
                WorldIds::Cobalt as WorldId,
                WorldIds::Jaeger as WorldId,
            ])),
            service: Service::Event,
        };
        let v = serde_json::to_value(input).unwrap();

        let expected = json!({
            "service": "event",
            "action": "subscribe",
            "eventNames": [
                "PlayerLogin",
                "MetagameEvent",
                "BattleRankUp",
                "FacilityControl",
                "ItemAdded",
                "VehicleDestroy",
                "PlayerFacilityCapture",
                "PlayerFacilityDefend",
                "SkillAdded",
                "GainExperience",
                "Death",
                "PlayerLogout"
            ],
            "characters": [ "all" ],
            "logicalAndCharactersWithWorlds": true,
            "worlds": [
                "13",
                "19"
            ]
        });

        assert_eq!(v, expected);
    }

    #[test]
    fn serialize_clearsubscribe_action() {
        let input = Action::ClearSubscribe {
            all: None,
            event_names: Some(EventSubscription::Ids(vec![
                EventNames::PlayerLogin,
                EventNames::MetagameEvent,
                EventNames::BattleRankUp,
                EventNames::FacilityControl,
                EventNames::ItemAdded,
                EventNames::VehicleDestroy,
                EventNames::PlayerFacilityCapture,
                EventNames::PlayerFacilityDefend,
                EventNames::SkillAdded,
                EventNames::GainExperience,
                EventNames::Death,
                EventNames::PlayerLogout,
            ])),
            characters: Some(CharacterSubscription::Ids(vec![1, 2])),
            worlds: Some(WorldSubscription::Ids(vec![
                WorldIds::Cobalt as WorldId,
                WorldIds::Jaeger as WorldId,
            ])),
            service: Service::Event,
        };
        let v = serde_json::to_value(input).unwrap();

        let expected = json!({
            "service": "event",
            "action": "clearSubscribe",
            "characters": [
                "1",
                "2"
		    ],
            "eventNames": [
                "PlayerLogin",
                "MetagameEvent",
                "BattleRankUp",
                "FacilityControl",
                "ItemAdded",
                "VehicleDestroy",
                "PlayerFacilityCapture",
                "PlayerFacilityDefend",
                "SkillAdded",
                "GainExperience",
                "Death",
                "PlayerLogout"
            ],
            "worlds": [
                "13",
                "19"
            ]
        });

        assert_eq!(v, expected);
    }

    #[test]
    fn serialize_clearsubscribe_all_action() {
        let input = Action::ClearSubscribe {
            all: Some(true),
            event_names: None,
            characters: None,
            worlds: None,
            service: Service::Event,
        };
        let v = serde_json::to_value(input).unwrap();

        let expected = json!({
            "service": "event",
            "action": "clearSubscribe",
            "all": "true"
        });

        assert_eq!(v, expected);
    }

    #[test]
    fn serialize_recentcharacterids_action() {
        let input = Action::RecentCharacterIds {
            service: Service::Event,
        };
        let v = serde_json::to_value(input).unwrap();

        let expected = json!({
            "service": "event",
            "action": "recentCharacterIds"
        });

        assert_eq!(v, expected);
    }

    #[test]
    fn serialize_recentcharacteridscount_action() {
        let input = Action::RecentCharacterIdsCount {
            service: Service::Event,
        };
        let v = serde_json::to_value(input).unwrap();

        let expected = json!({
            "service": "event",
            "action": "recentCharacterIdsCount"
        });

        assert_eq!(v, expected);
    }

}
