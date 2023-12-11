use std::time::{SystemTime, UNIX_EPOCH};

use crate::{schematic::Schematic, component::*, electronic_calculations::*, output::*};
mod schematic;
mod component;
mod extra_functions;
mod electronic_calculations;
mod output;

fn main() {
    
    /*let mut schematic = Schematic::new("test");
    
    let mut resistor1 = Component::init(
        Components::Resistor,  1.0, 
        CompPrefix::KILO, false, 0);
    let mut resistor2 = Component::init(Components::Resistor, 1.0, CompPrefix::KILO, true, 0);
    let mut resistor3 = Component::init(Components::Resistor, 1.0, CompPrefix::KILO, false, 0);
    let mut resistor4 = Component::init(Components::Resistor, 1.0, CompPrefix::KILO, false, 0);
    let mut resistor5 = Component::init(Components::Resistor, 1.0, CompPrefix::KILO, false, 0);
    let mut capacitor6 = Component::init(Components::Capacitor, 10.0, CompPrefix::MICRO, false, 0);
    
    let mut battery = Component::init(
        Components::Battery, 12.0, 
        CompPrefix::NONE, false, 0);
    schematic.add(&mut battery);
    schematic.add(&mut resistor1);
    schematic.add(&mut resistor2);
    schematic.add(&mut resistor3);
    schematic.add(&mut resistor4);
    schematic.add(&mut resistor5);
    schematic.add(&mut capacitor6);
    
    
    schematic.connect(resistor1, 0, battery, 1);
    schematic.connect(resistor2, 0, battery, 1);
    
    schematic.connect(resistor3, 0, resistor1, 1);
    schematic.connect(resistor4, 0, resistor1, 1);
    schematic.connect(resistor3, 1, resistor4, 1);

    schematic.connect(resistor2, 1, resistor3, 1);
    schematic.connect(resistor5, 0, resistor2, 1);

    schematic.connect(resistor5, 1, battery, 0);
    schematic.connect(resistor1, 1, capacitor6, 0);
    schematic.connect(capacitor6, 1, battery, 0);

    schematic.compile();*/
    let output = Output::create("output_test");
    let mut dechandler = DECHandler::init(TimeMode::Millis);
    let mut count: u128 = 0;
    dechandler.set_start_time(0);

    let mut mode = true;
    let mut val:f64 = 0.0;
    let mut last_val:f64 = 0.0;
    loop {
        dechandler.update(count);
        if count > 200 && mode==true{
            last_val = val;
            mode = false;
        }
        if count > 2000 {
            break;
        }
        if mode {
            val = ECalculation::rc_charge(10.0, CompPrefix::KILO, 10.0, CompPrefix::MICRO, count, TimeMode::Millis, 12.0);
        }else {
            val = ECalculation::rc_discharge(20.0, CompPrefix::KILO, 10.0, CompPrefix::MICRO, count-200, TimeMode::Millis, last_val);
        }
        
        output.write_line(val, count as f64);
        count+=1;
    }

    /*let volt = ECalculation::ohm_law(Some(100.0), None, Some(20.0));
    print!("{}", volt);*/
}
