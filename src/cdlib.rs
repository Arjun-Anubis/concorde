pub mod objects;
pub mod defaults;


pub fn test() -> serde_json::Result<()>{
    let props = defaults::get_default_network_props();
    let data = objects::Data::Identify( objects::Identify { token : String::from("abc"), intents : 512, properties:  props } );
    let event =  objects::SendEvent { op : 3, d : data };
    println!( "{}", event.to_string() );
    Ok(())
}

pub fn run() -> Result<(), &'static str> {
    pretty_env_logger::init();
    let (mut socket, response) = tungstenite::connect(
        url::Url::parse("wss://gateway.discord.gg")
        .unwrap())
        .expect("Can't connect");

    log::debug!( "Connected with {}", response.status() );

    loop {
        let msg = socket.read_message().expect("Error reading message");
        match event_handler( msg ) {
            Ok(message) => {
                log::debug!( "Handled succesfuly, to send {}", message );
                match socket.write_message( message ) {
                    Ok(()) => log::debug!( "Succesfuly wrote" ),
                    Err(e) => {
                        if let tungstenite::Error::ConnectionClosed = e {
                            log::warn!( "Connection has been closed." );
                            break
                        } else {
                            log::warn!( "Message too large" )
                        }
                        log::debug!( "There was an error sending the message" )
                    }
                }
            }
            Err(e) => { log::debug!( "Failed with err code {}", e ); break }
        }
    };

    match socket.close(None) {
        Ok(()) => log::info!( "Closed successfuly" ),
        Err(..) => log::error!( "Error while closing" )
    };

    Ok(())
}

fn event_handler( msg: tungstenite::Message ) -> Result<tungstenite::Message, u32> { 
    log::info!( "Handling {:?}", msg );
    Ok( tungstenite::Message::Text( "Good".to_owned() ) )
}
