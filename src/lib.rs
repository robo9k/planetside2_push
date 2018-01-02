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

pub enum Service {
    Event,
    Push,
}

pub enum CharacterSubscription {
    All,
    Ids(HashSet<CharacterId>),
}

pub enum WorldSubscription {
    All,
    Ids(HashSet<WorldId>),
}

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

pub enum EventSubscription {
    All,
    Ids(HashSet<EventNames>),
}

enum Action {
    /*
    Echo {
        service: Service,
    },
    */
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
    /*
    RecentCharacterIds {
        service: Service,
    },
    RecentCharacterIdsCount {
        service: Service,
    },
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
