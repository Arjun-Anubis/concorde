
pub mod objects;
pub mod defaults {
   pub fn get_default_network_props() -> super::objects::NetworkProperties { 
        super::objects::NetworkProperties { os: "linux".to_owned(), browser: "linux".to_owned(), device: "linux".to_owned() }
   }
}


pub fn test() -> serde_json::Result<()>{
    let props = defaults::get_default_network_props();
    let data = objects::Data::Identify( objects::Identify { token : String::from("abc"), intents : 512, properties:  props } );
    let event =  objects::Event { op : 3, d : data };
    println!( "{}", event.to_string() );
    Ok(())
}

pub fn run() -> Result<(), &'static str> {
    let (mut socket, response) = tungstenite::connect(
        url::Url::parse("wss://gateway.discord.gg")
        .unwrap())
        .expect("Can't connect");

    println!( "Connected with {}", response.status() );

    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }

    match socket.close(None) {
        Ok(()) => println!( "Closed successfuly" ),
        Err(..) => println!( "Error while closing" )
    }

    Ok(())
}
