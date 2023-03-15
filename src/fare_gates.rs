use phf::{phf_ordered_map, OrderedMap};

pub enum SubwayLine {
    RedLine,
    OrangeLine,
    GreenLine,
    BlueLine,
    SilverLine,
}

pub struct Station {
    name: StationName,
    subway_lines: Vec<SubwayLine>,
}

#[derive(Eq, PartialEq, Hash)]
pub enum StationName {
    OakGrove,
    MaldenCenter,
}

//const FARE_GATE_IDS: OrderedMap<Stations, u16> = phf_ordered_map! {
//Stations::OakGrove => 0,
//Stations::MaldenCenter => 1,
//};

//pub fn station_from_gate_id(gate_id: u16)
//const FARE_GATE_IDS: HashMap<&str, HashMap<u16, u16>> = HashMap::from([
//("Oak Grove", HashMap::from([(0, 1), (1, 2)])),
//("Malden Center", HashMap::from([(0, 1), (1, 2)])),
//]);
