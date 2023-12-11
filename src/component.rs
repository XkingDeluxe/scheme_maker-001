#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Components {
    Resistor,
    Capacitor,
    Battery,
    VoltageMeter,
    AmperageMeter
}
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum CompPrefix {
    PETA,
    TERA,
    GIGA,
    MEGA,
    KILO,
    HECTO,
    DECA,
    NONE,
    DECI,
    CENTI,
    MILLI,
    MICRO,
    NANO,
    PICO,
    FEMTO
}
impl CompPrefix {
    pub fn getFactor(p: CompPrefix) -> f64{
        match p {
            CompPrefix::PETA => 1000000000000000.0,
            CompPrefix::TERA => 1000000000000.0,
            CompPrefix::GIGA => 1000000000.0,
            CompPrefix::MEGA => 1000000.0,
            CompPrefix::KILO => 1000.0,
            CompPrefix::HECTO => 100.0,
            CompPrefix::DECA => 10.0,
            CompPrefix::NONE => 1.0,
            CompPrefix::DECI => 0.1,
            CompPrefix::CENTI => 0.01,
            CompPrefix::MILLI => 0.001,
            CompPrefix::MICRO => 0.000001,
            CompPrefix::NANO => 0.000000001,
            CompPrefix::PICO => 0.000000000001,
            CompPrefix::FEMTO => 0.0000000000001,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum ComponentHold{
    Type(Components),
    DynamicIndex(u8),
    Value(f64),
    Pins([u8; 2])
}
#[derive(Debug, Copy, Clone)]
pub struct Component{
    pub comp_type: Components,
    pub value: f32,
    pub prefix: CompPrefix,
    pub dynamic: bool,
    pub dynamic_index: u8,
    pub index: u8
}

impl Component {
    pub fn init(_comp_type: Components, _value: f32, _prefix: CompPrefix, _dynamic: bool, _dynamic_index: u8) -> Component{
        Component { comp_type: _comp_type, value: _value, prefix: _prefix, dynamic: _dynamic, dynamic_index: _dynamic_index, index: 0}
    }
}