extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::collections::HashSet;

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
pub enum CharacterSubscription {
    All,
    Ids(HashSet<CharacterId>),
}

#[derive(Serialize)]
pub enum WorldSubscription {
    All,
    Ids(HashSet<WorldId>),
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
pub enum EventSubscription {
    All,
    Ids(HashSet<EventNames>),
}

#[derive(Serialize)]
#[serde(tag = "action", rename_all = "camelCase")]
enum Action {
    Echo {
        payload: serde_json::Value,
        service: Service,
    },
    Subscribe {
        event_names: Option<EventSubscription>,
        characters: Option<CharacterSubscription>,
        logical_and_characters_with_worlds: Option<bool>,
        worlds: Option<WorldSubscription>,
        service: Service,
    },
    ClearSubscribe {
        all: Option<bool>,
        event_names: Option<EventSubscription>,
        characters: Option<CharacterSubscription>,
        worlds: Option<WorldSubscription>,
        service: Service,
    },
    RecentCharacterIds {
        service: Service,
    },
    RecentCharacterIdsCount {
        service: Service,
    },
}

#[cfg(test)]
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
    fn serialize_recentcharaterids_action() {
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
    fn serialize_recentcharateridscount_action() {
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
