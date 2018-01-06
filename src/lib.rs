extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate websocket;

pub mod request;

pub struct ServiceId<'a>(&'a str);

impl<'a> ServiceId<'a> {
    const EXAMPLE_ID: &'static str = "s:example";

    pub const EXAMPLE: ServiceId<'static> = ServiceId(ServiceId::EXAMPLE_ID);

    // TODO: FromStr
    pub fn new(service_id: &str) -> Result<ServiceId, ()> {
        if service_id.starts_with("s:") {
            Ok(ServiceId(service_id))
        } else {
            Err(())
        }
    }

    pub fn is_example(&self) -> bool {
        ServiceId::EXAMPLE_ID == self.0
    }
}

impl<'a> AsRef<str> for ServiceId<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

pub enum Environment {
    Pc,
    Ps4Us,
    Ps4Eu,
}

impl AsRef<str> for Environment {
    fn as_ref(&self) -> &str {
        use Environment::*;

        match *self {
            Pc => "ps2",
            Ps4Us => "ps2ps4us",
            Ps4Eu => "ps2ps4eu",
        }
    }
}

fn websocket_endpoint(env: Environment, sid: &ServiceId) -> websocket::url::Url {
    websocket::url::Url::parse_with_params(
        "wss://push.planetside2.com/streaming",
        &[("environment", env.as_ref()), ("service-id", sid.as_ref())],
    ).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serviceid_new() {
        assert!(ServiceId::new("s:example").is_ok());
        assert!(ServiceId::new("error").is_err());
    }

    #[test]
    fn serviceid_example() {
        assert!(ServiceId::EXAMPLE.is_example());

        let example_sid = ServiceId::new("s:example").unwrap();
        assert!(example_sid.is_example());

        let not_example_sid = ServiceId::new("s:not_example").unwrap();
        assert!(!not_example_sid.is_example());
    }

    #[test]
    fn websocket_endpoint() {
        let address = super::websocket_endpoint(Environment::Pc, &ServiceId::EXAMPLE);
        let expected = websocket::url::Url::parse(
            "wss://push.planetside2.com/streaming?environment=ps2&service-id=s:example",
        ).unwrap();
        assert_eq!(address, expected);
    }
}
