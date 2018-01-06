extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod request;

pub struct ServiceId<'a>(&'a str);

impl<'a> ServiceId<'a> {
    const EXAMPLE_ID: &'static str = "s:example";

    pub const EXAMPLE: ServiceId<'static> = ServiceId(ServiceId::EXAMPLE_ID);

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
}
