pub mod objects;
pub mod defaults;


pub fn test() -> serde_json::Result<()>{
    let props = defaults::get_default_network_props();
    let data =  objects::send::Identify { token : String::from("abc"), intents : 512, properties:  props };
    let event =  objects::send::Event { op : 3, d : data };
    println!( "{}", event.to_string() );
    let heartbeat = defaults::get_heartbeat();
    println!( "{}", heartbeat.to_string() );
    Ok(())
}

pub struct Client
{
    pub token: String,
    // Other things
}
impl Client {

    pub fn run( &self ) -> Result<(), &'static str> {
        pretty_env_logger::init();
        let (mut socket, response) = tungstenite::connect(
            url::Url::parse("wss://gateway.discord.gg")
            .unwrap())
            .expect("Can't connect");

        log::debug!( "Connected with {}", response.status() );

        loop {
            match socket.read_message() {
                Ok(msg) => {
                    match self.event_handler( msg ) {
                        Ok(message) => {
                            log::debug!( "Handled succesfuly, to send {}", message );
                            match socket.write_message( message ) {
                                Ok(()) => log::debug!( "Succesfuly wrote" ),
                                Err(e) => {
                                    log::debug!( "There was {} sending the message", e )
                                }
                            }
                        }
                        Err(e) =>
                        {
                            match e {
                                "No send" => log::info!("Nothing to send"),
                                "Empty" => break,
                                _ => log::error!("Event Handler failed with {}", e)
                            }
                        }
                    }
                }
                Err(e) => {
                    if let tungstenite::Error::ConnectionClosed = e {
                        log::warn!( "Connection has been closed." );
                        break
                    } else {
                        log::warn!( "Message too large" )
                    }
                }
            }
        };

        match socket.close(None) {
            Ok(()) => log::info!( "Closed successfuly" ),
            
            Err(..) => log::error!( "Error while closing" )
        };

        Ok(())
    }
    fn on_message( &self, event: objects::recv::Event<objects::recv::dispatch::Message> ) -> Result<(), &'static str> {
        log::info!( "Handling message(default)" );
        Ok(())
    }

    fn event_handler( &self, msg: tungstenite::Message ) -> Result<tungstenite::Message, &'static str> { 
        if msg.is_empty() { return Err("Empty") };
        match msg.into_text() {
            Ok(content) => {
                log::trace!( "{:#}", content);
                let fragmented_event: serde_json::Value = serde_json::from_str(&content).unwrap();
                let opcode = fragmented_event["op"].as_i64().unwrap();

                match opcode {
                    0 => {
                        let t = fragmented_event["t"].as_str().unwrap();
                        // Dipatch types
                        match t {
                            "READY" => {
                                let res: Result<objects::recv::Event<objects::recv::dispatch::Ready>, _> = serde_json::from_str(content.as_str());
                                match res {
                                    Ok(event) =>  {
                                        log::info!( "Decoded to {:}", event.to_string() );
                                        Err( "No send" )
                                    }
                                    Err(..) => {
                                        Err( "Could not serialize" )
                                    }
                                }
                            }
                            "MESSAGE_CREATE" =>
                            {
                                log::info!( "{}", content );
                                let res: Result<objects::recv::Event<objects::recv::dispatch::Message>, _> = serde_json::from_str(content.as_str());
                                match res {
                                    Ok(event) =>  {
                                        log::info!( "Decoded to {:}", serde_json::to_string( &event ).unwrap() );
                                        match &self.on_message( event ) {
                                            Ok(()) => Err("No send"),
                                            Err(e) => Err(e)
                                        }
                                    }
                                    Err(e) => {
                                        log::error!( "Error was {}", e );
                                        Err( "Could not serialize" )
                                    }
                                }

                            }
                            _ => {
                                log::warn!( "Unhandled dispatch" );
                                Err("Unhandled Dispatch")
                            }
                        }
                        }
                    7 => {
                        // Reconnect
                        Err( "Reconnect" )
                    }
                    9 => {
                        // Session Invalid
                        Err( "Session Invalid" )
                    }

                    10 => {
                        let res: Result<objects::recv::Event<objects::recv::Hello>, _> = serde_json::from_str(content.as_str());
                        match res {
                            Ok(event) => {
                                log::info!( "Decoded to {:?}", event );
                                let send_event = defaults::get_default_identify(&self.token);
                                Ok( tungstenite::Message::Text( send_event.to_string() ) )
                            }
                            Err(_e) => {
                                Err( "Could not serialize" )
                            }
                        }
                    }
                    11 => {
                        // Heartbeat Ack
                        // Err but not really, Ok, would have been sent
                        Err( "No send" )
                    }
                    _ => Err("Unrecognised Opcode")
                }
            }
            Err(_e) => {
                Err( "Could not convert to string" )
            }
        }
    }
}
