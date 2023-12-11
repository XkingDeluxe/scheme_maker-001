use libm::powf;

use crate::component::CompPrefix;

const EULER: f32 = 2.71828;

pub enum TimeMode{
   Seconds,
   Millis,
   Micros,
   Nanos 
}
impl TimeMode {
    pub fn getFactor(t: TimeMode) -> f64{
        match t {
            TimeMode::Seconds => 1.0,
            TimeMode::Millis => 0.001,
            TimeMode::Micros => 0.000001,
            TimeMode::Nanos => 0.0000000001
        }
    }
}
pub struct DECHandler{
    mode: TimeMode,
    prev_time: u128,
    pub delta_time: u128,

}
impl DECHandler {
    pub fn init(mode_: TimeMode) -> DECHandler{
        DECHandler {mode: mode_, prev_time: 0, delta_time: 0}
    }
    pub fn set_start_time(&mut self, time:u128){
        self.prev_time= time;
    }
    pub fn update(&mut self, time:u128){
        self.delta_time = time-self.prev_time;
        self.prev_time = time;
    }
}

pub struct ECalculation{

}
impl ECalculation{
    pub fn ohm_law(r: Option<f64>, u: Option<f64>, i: Option<f64>) -> f64{
        if r.is_some() && u.is_some() && i.is_some() {
            panic!("Should only provide 2 quantities")
        }
        if r.is_some() && u.is_some() && i.is_none() {
            return u.unwrap()/r.unwrap();
        }
        if r.is_some() && u.is_none() && i.is_some() {
            return i.unwrap()*r.unwrap();
        }
        if r.is_none() && u.is_some() && i.is_none() {
            return u.unwrap()/i.unwrap();
        }
        panic!("Should provide 2 quantities");
    }
    pub fn rc_discharge(r: f64, rp: CompPrefix, c: f64, cp: CompPrefix, t: u128, m: TimeMode, u: f64) -> f64{
        let exponent: f64 = -(t as f64 * TimeMode::getFactor(m))/(r*CompPrefix::getFactor(rp)*c*CompPrefix::getFactor(cp));
        u * powf(EULER, exponent as f32) as f64
    }
    pub fn rc_charge(r: f64, rp: CompPrefix, c: f64, cp: CompPrefix, t: u128, m: TimeMode, u: f64) -> f64{
        let exponent: f64 = -(t as f64 * TimeMode::getFactor(m))/(r*CompPrefix::getFactor(rp)*c*CompPrefix::getFactor(cp));
        u * (1.0 - powf(EULER, exponent as f32) as f64)
    }
}