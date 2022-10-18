use cdlib;

fn main() {
    dotenv::dotenv().ok();
    let token = std::env::var("token").expect( "Must set token" );
    let client = cdlib::Client{ token };
    match client.run() {
        Ok(()) => println!( "Closed Successfully" ),
        Err(..) => println!( "There was an error" )
        }
}
