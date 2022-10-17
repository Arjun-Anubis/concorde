use cdlib;

fn main() {
    match cdlib::run() {
        Ok(()) => println!( "Connected Successfully" ),
        Err(..) => println!( "There was an error" )
        }
}
