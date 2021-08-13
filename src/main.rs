use std::{thread, time};
use structopt::StructOpt;
use std::fs::File;
use std::io::BufReader;
use rodio::Source;
#[derive(StructOpt)]
struct Cli {
    /// The treshold
    treshold: f32,
}
fn calculate_treshold(reference:f32,deviation:f32)-> f32{
    return reference+ reference*deviation/100.0
}



fn print_treshold(treshold:f32) {

    println!("soglia inferiore: {:?}",calculate_treshold(treshold,-10.0));
    println!("soglia superiore: {:?}",calculate_treshold(treshold,10.0));
}
use battery::units::ratio::percent;





fn main() -> Result<(), battery::Error> {
    let five_minutes = time::Duration::from_millis(300000);
    let args = Cli::from_args();
    println!("treshold: {:?}",args.treshold);
    let manager = battery::Manager::new()?;
    let treshold = args.treshold;
    let mut  connected = true;

loop{
    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        //let  charge = battery.state_of_charge()*100.0;
        println!("Battery #{}:", idx);
        println!("Vendor: {:?}", battery.vendor());
        println!("Model: {:?}", battery.model());
        println!("State: {:?}", battery.state());
        println!("Time to full charge: {:?}", battery.time_to_full());
        let charge = battery.state_of_charge().get::<percent>();


    //charge = battery.state_of_charge()*100.0;
    println!("percentuale di carica: {:.2?}",charge);

    print_treshold(treshold);
    match charge {
        charge if charge < calculate_treshold(treshold,-10.0) => connected =  true, // println!("connected"),
        charge if charge > calculate_treshold(treshold,10.0) => connected = false, //println!("disconnected"),
        _=>  print_treshold(treshold), // se compreso alimentazione connessa
    }
    if connected
    {
        println!("Connesso");
        if battery.state()!= battery::State::Charging{

        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let file = File::open("mooh.ogg").unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        stream_handle.play_raw(source.convert_samples()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    }
    else
    {
        println!("disconnesso");
        if battery.state()!=battery::State::Discharging{

        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let file = File::open("carica.ogg").unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        stream_handle.play_raw(source.convert_samples()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
    }


    }
    thread::sleep(five_minutes);

                                                                  }
} // end loop
} // end main
