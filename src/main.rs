

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn calculate_treshold(reference:f32,deviation:f32)-> f32{
    return reference+ reference*deviation/100.0
}
use battery::units::ratio::percent;
fn main() -> Result<(), battery::Error> {
    let manager = battery::Manager::new()?;
    let treshold = 50;


    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        let charge = battery.state_of_charge()*100.0;
        println!("Battery #{}:", idx);
        println!("Vendor: {:?}", battery.vendor());
        println!("Model: {:?}", battery.model());
        println!("State: {:?}", battery.state());
        println!("Time to full charge: {:?}", battery.time_to_full());
        println!("percentuale di carica: {:.2?}",charge);
        let charge = battery.state_of_charge().get::<percent>();



match charge {
    charge if charge < calculate_treshold(90.0,-10.0) => println!("on"),
    charge if charge > calculate_treshold(90.0,10.0) => println!("off"),
    _=> println!("not covered")
}






    }

    Ok(())
}
