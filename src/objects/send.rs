#[derive(serde::Serialize)]
pub struct Event<DataType> {
    pub op : i32,
    pub d :  DataType,
}

impl<DataType: serde::Serialize> Event<DataType> {
    pub fn to_string( &self ) -> String {
        match serde_json::to_string_pretty( &self ) {
            Ok(s) => s,
            Err(..) => "".to_owned() 
        }
    }
}



#[derive(serde::Serialize)]
pub struct Identify {
    pub token: String,
    pub intents: i32,
    pub properties: NetworkProperties
}


#[derive(serde::Serialize)]
pub struct NetworkProperties {
    pub os: String,
    pub browser: String,
    pub device: String
}

#[derive(serde::Serialize)]
pub struct Resume {
}

#[derive(serde::Serialize)]
pub struct VoiceStateUpdate {

}
pub type Heartbeat = i32;

#[derive(serde::Serialize)]
pub struct RequestGuildMembers {

}
