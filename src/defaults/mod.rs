use super::objects;

pub fn get_default_network_props() -> objects::send::NetworkProperties { 
    objects::send::NetworkProperties { os: "linux".to_owned(), browser: "concorde v1.0.0".to_owned(), device: "concorde v1.0.0".to_owned() }
}
pub fn get_default_identify(token: &String) -> objects::send::Event<objects::send::Identify>{
    super::objects::send::Event
        { op: 2,
        d : objects::send::Identify {
            intents: 512,
            token: token.to_string(),
            properties: get_default_network_props()
            }
        } 
}

pub fn get_heartbeat() -> objects::send::Event<objects::send::Heartbeat> {
    objects::send::Event {
        op: 1,
        d: rand::random()
    }
}
