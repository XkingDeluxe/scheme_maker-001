use crate::{schematic::Schematic, component::*};
mod schematic;
mod component;
mod extra_functions;

fn main() {
    
    let mut schematic = Schematic::new("test.txt");
    
    let mut resistor1 = Component::init(Components::Resistor, 1.0, CompPrefix::KILO, false, 0);
    let mut resistor2 = Component::init(Components::Resistor, 1.0, CompPrefix::KILO, true, 0);
    let mut resistor3 = Component::init(Components::Resistor, 1.0, CompPrefix::KILO, false, 0);
    let mut resistor4 = Component::init(Components::Resistor, 1.0, CompPrefix::KILO, false, 0);
    let mut resistor5 = Component::init(Components::Resistor, 1.0, CompPrefix::KILO, false, 0);

    
    let mut battery = Component::init(Components::Battery, 12.0, CompPrefix::NONE, false, 0);
    schematic.add(&mut battery);
    schematic.add(&mut resistor1);
    schematic.add(&mut resistor2);
    schematic.add(&mut resistor3);
    schematic.add(&mut resistor4);
    schematic.add(&mut resistor5);
    
    
    schematic.connect(resistor1, 0, battery, 1);
    schematic.connect(resistor2, 0, battery, 1);

    schematic.connect(resistor3, 0, resistor1, 1);
    schematic.connect(resistor4, 0, resistor1, 1);
    schematic.connect(resistor3, 1, resistor4, 1);

    schematic.connect(resistor2, 1, resistor4, 1);

    schematic.connect(resistor2, 1, resistor5, 0);

    schematic.connect(resistor5, 1, battery, 0);


    schematic.compile();
}
