use serde::Serialize;

#[derive(Serialize)]
pub struct Event {
    pub op : u32,
    pub d :  Data,
}

impl Event {
    pub fn to_string( self ) -> String {
        match serde_json::to_string_pretty( &self ) {
            Ok(s) => s,
            Err(..) => "".to_owned() 
        }
    }
}

#[derive(Serialize)]
pub enum Data {
    Heartbeat(u32), // u32
    Identify(Identify),
    VoiceStateUpdate(VoiceStateUpdate),
    RequestGuildMembers
}

#[derive(Serialize)]
pub struct Identify {
    pub token: String,
    pub intents: u32,
    pub properties: NetworkProperties
}


#[derive(Serialize)]
pub struct NetworkProperties {
    pub os: String,
    pub browser: String,
    pub device: String
}

#[derive(Serialize)]
pub struct VoiceStateUpdate {

}


