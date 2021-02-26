fn main() -> Result<(), battery::Error> {
    let manager = battery::Manager::new()?;
    //let percentage =

 //impl Mul for uom::si::Quantity{
    // fn Mul(fattore:i32){
    //     println!("moltiplico per {}",fattore );
     //}
 //}

    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        println!("Battery #{}:", idx);
        println!("Vendor: {:?}", battery.vendor());
        println!("Model: {:?}", battery.model());
        println!("State: {:?}", battery.state());
        println!("Time to full charge: {:?}", battery.time_to_full());
        println!("percentuale di carica: {:.2?}",(battery.state_of_charge()*100.0));


        println!("");


    }

    Ok(())
}
