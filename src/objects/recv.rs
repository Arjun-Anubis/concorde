pub mod dispatch;
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Event<DataType> {
    pub op : i32,
    pub d : Option<DataType>,
    pub s: Option<i32>,
    pub t: Option<String>
}

impl<DataType: serde::Serialize> Event<DataType> {
    pub fn to_string( self ) -> String {
        match serde_json::to_string_pretty( &self ) {
            Ok(s) => s,
            Err(..) => "".to_owned() 
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)] 
pub struct Hello {
    pub heartbeat_interval: i32
}
