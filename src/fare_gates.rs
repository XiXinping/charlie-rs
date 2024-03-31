use phf::{phf_ordered_map, OrderedMap};
use serde::Serialize;
use std::fmt;
use strum;
use strum_macros::Display;
use strum_macros::EnumString;

/// The location of a fare gate. Can either be a subway station or a particular bus
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Location {
    Bus(u16),
    Station(Station),
    Unknown,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        match self {
            Location::Bus(number) => write!(f, "Bus #{number}"),
            Location::Station(station) => {
                if station.subway_lines.len() == 1 {
                    write!(f, "{} - ", station.subway_lines[0]);
                } else if station.subway_lines.len() == 2 {
                    write!(
                        f,
                        "{} and {} - ",
                        station.subway_lines[0], station.subway_lines[1]
                    );
                } else if station.subway_lines.len() == 3 {
                    write!(
                        f,
                        "{}, {}, and {} - ",
                        station.subway_lines[0], station.subway_lines[1], station.subway_lines[2]
                    );
                }
                write!(f, "{station}")
            }
            Location::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Location {
    pub fn from_machine_id(gate_id: u16) -> Location {
        if let Some(location) = FARE_GATE_IDS.get(&gate_id) {
            *location
        } else {
            Location::Unknown
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Station {
    name: StationName,
    subway_lines: &'static [SubwayLine],
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "{}", self.name)
    }
}

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Serialize)]
#[strum(serialize_all = "title_case")]
pub enum SubwayLine {
    BlueLine,
    GreenLine,
    OrangeLine,
    RedLine,
    SilverLine,
}

#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Hash, Serialize)]
#[strum(serialize_all = "title_case")]
pub enum StationName {
    Alewife,
    Davis,
    Porter,
    Harvard,
    Central,
    #[strum(serialize = "Kendall/MIT")]
    KendallMit,
    #[strum(serialize = "Charles/MGH")]
    CharlesMgh,
    ParkStreet,
    Broadway,
    Andrew,
    #[strum(serialize = "JFK/UMass")]
    JfkUmass,
    SavinHill,
    FieldsCorner,
    Shawmut,
    Ashmont,
    NorthQuincy,
    Wollaston,
    QunicyCenter,
    QuincyAdams,
    Braintree,
    OakGrove,
    MaldenCenter,
    Wellington,
    Assembly,
    SullivanSquare,
    CommunityCollege,
    NorthStation,
    Haymarket,
    State,
    DowntownCrossing,
    Chinatown,
    TuftsMedicalCenter,
    BackBay,
    MassachusettsAvenue,
    Ruggles,
    RoxburyCrossing,
    JacksonSquare,
    GreenStreet,
    ForestHills,
    Bowdoin,
    GovernmentCenter,
    Aquarium,
    Maverick,
    Airport,
    WoodIsland,
    OrientHeights,
    SuffolkDowns,
    Beachmont,
    RevereBeach,
    Wonderland,
    SouthStation,
    Courthouse,
}

const FARE_GATE_IDS: OrderedMap<u16, Location> = phf_ordered_map! {
    // Davis
    6766u16 => Location::Station(Station {name: StationName::Davis, subway_lines: &[SubwayLine::RedLine]}),
    6767u16 => Location::Station(Station {name: StationName::Davis, subway_lines: &[SubwayLine::RedLine]}),
    6768u16 => Location::Station(Station {name: StationName::Davis, subway_lines: &[SubwayLine::RedLine]}),
    6769u16 => Location::Station(Station {name: StationName::Davis, subway_lines: &[SubwayLine::RedLine]}),
    6770u16 => Location::Station(Station {name: StationName::Davis, subway_lines: &[SubwayLine::RedLine]}),
    6771u16 => Location::Station(Station {name: StationName::Davis, subway_lines: &[SubwayLine::RedLine]}),
    6772u16 => Location::Station(Station {name: StationName::Davis, subway_lines: &[SubwayLine::RedLine]}),
    2167u16 => Location::Station(Station {name: StationName::Davis, subway_lines: &[SubwayLine::RedLine]}),
    7020u16 => Location::Station(Station {name: StationName::Davis, subway_lines: &[SubwayLine::RedLine]}),
    // Porter
    6781u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    6780u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    6779u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    6778u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    6777u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    6776u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    6775u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    2168u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    7021u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    6782u16 => Location::Station(Station {name: StationName::Porter, subway_lines: &[SubwayLine::RedLine]}),
    // Oak Grove
    6640u16 => Location::Station(Station {name: StationName::OakGrove, subway_lines: &[SubwayLine::OrangeLine]}),
    6641u16 => Location::Station(Station {name: StationName::OakGrove, subway_lines: &[SubwayLine::OrangeLine]}),
    6639u16 => Location::Station(Station {name: StationName::OakGrove, subway_lines: &[SubwayLine::OrangeLine]}),
    2036u16 => Location::Station(Station {name: StationName::OakGrove, subway_lines: &[SubwayLine::OrangeLine]}),
    6642u16 => Location::Station(Station {name: StationName::OakGrove, subway_lines: &[SubwayLine::OrangeLine]}),
    6979u16 => Location::Station(Station {name: StationName::OakGrove, subway_lines: &[SubwayLine::OrangeLine]}),
    // Downtown Crossing
    2091u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    6995u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    6699u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    6700u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    1926u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    2084u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    6994u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    6695u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    6694u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    6696u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    2336u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    1056u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    6814u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    6813u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    2212u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    7038u16 => Location::Station(Station {name: StationName::DowntownCrossing, subway_lines: &[SubwayLine::OrangeLine, SubwayLine::RedLine]}),
    // State
    7092u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    1844u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    6689u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    6988u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    6991u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    2083u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    6688u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    6687u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    6686u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    2078u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    6987u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    7090u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    1842u16 => Location::Station(Station {name: StationName::State, subway_lines: &[SubwayLine::BlueLine, SubwayLine::OrangeLine]}),
    // Haymarket
    6684u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6683u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6682u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6681u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    2073u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    7074u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6883u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6884u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6885u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6886u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    2303u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6986u16 => Location::Station(Station {name: StationName::Haymarket, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    // North Station
    6985u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    2063u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6671u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6672u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6673u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6674u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6675u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6676u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6677u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6678u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6984u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    2062u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6668u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6667u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6666u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6665u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    6664u16 => Location::Station(Station {name: StationName::NorthStation, subway_lines: &[SubwayLine::GreenLine, SubwayLine::OrangeLine]}),
    // Sullivan Square
    6654u16 => Location::Station(Station {name: StationName::SullivanSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6655u16 => Location::Station(Station {name: StationName::SullivanSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6656u16 => Location::Station(Station {name: StationName::SullivanSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6657u16 => Location::Station(Station {name: StationName::SullivanSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6658u16 => Location::Station(Station {name: StationName::SullivanSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6659u16 => Location::Station(Station {name: StationName::SullivanSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    2053u16 => Location::Station(Station {name: StationName::SullivanSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6982u16 => Location::Station(Station {name: StationName::SullivanSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    // Community College
    6661u16 => Location::Station(Station {name: StationName::CommunityCollege, subway_lines: &[SubwayLine::OrangeLine]}),
    6662u16 => Location::Station(Station {name: StationName::CommunityCollege, subway_lines: &[SubwayLine::OrangeLine]}),
    2056u16 => Location::Station(Station {name: StationName::CommunityCollege, subway_lines: &[SubwayLine::OrangeLine]}),
    6983u16 => Location::Station(Station {name: StationName::CommunityCollege, subway_lines: &[SubwayLine::OrangeLine]}),
    // Assembly
    3876u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    3875u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    6957u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    6956u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    6955u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    6954u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    6953u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    7101u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    3873u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    3872u16 => Location::Station(Station {name: StationName::Assembly, subway_lines: &[SubwayLine::OrangeLine]}),
    // Wellington
    6981u16 => Location::Station(Station {name: StationName::Wellington, subway_lines: &[SubwayLine::OrangeLine]}),
    2042u16 => Location::Station(Station {name: StationName::Wellington, subway_lines: &[SubwayLine::OrangeLine]}),
    6650u16 => Location::Station(Station {name: StationName::Wellington, subway_lines: &[SubwayLine::OrangeLine]}),
    6651u16 => Location::Station(Station {name: StationName::Wellington, subway_lines: &[SubwayLine::OrangeLine]}),
    6652u16 => Location::Station(Station {name: StationName::Wellington, subway_lines: &[SubwayLine::OrangeLine]}),
    6653u16 => Location::Station(Station {name: StationName::Wellington, subway_lines: &[SubwayLine::OrangeLine]}),
    // Malden
    6980u16 => Location::Station(Station {name: StationName::MaldenCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    2037u16 => Location::Station(Station {name: StationName::MaldenCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    6645u16 => Location::Station(Station {name: StationName::MaldenCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    6646u16 => Location::Station(Station {name: StationName::MaldenCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    6647u16 => Location::Station(Station {name: StationName::MaldenCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    6648u16 => Location::Station(Station {name: StationName::MaldenCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    // Chinatown
    6704u16 => Location::Station(Station {name: StationName::Chinatown, subway_lines: &[SubwayLine::OrangeLine]}),
    6705u16 => Location::Station(Station {name: StationName::Chinatown, subway_lines: &[SubwayLine::OrangeLine]}),
    2099u16 => Location::Station(Station {name: StationName::Chinatown, subway_lines: &[SubwayLine::OrangeLine]}),
    7003u16 => Location::Station(Station {name: StationName::Chinatown, subway_lines: &[SubwayLine::OrangeLine]}),
    7002u16 => Location::Station(Station {name: StationName::Chinatown, subway_lines: &[SubwayLine::OrangeLine]}),
    2096u16 => Location::Station(Station {name: StationName::Chinatown, subway_lines: &[SubwayLine::OrangeLine]}),
    6702u16 => Location::Station(Station {name: StationName::Chinatown, subway_lines: &[SubwayLine::OrangeLine]}),
    6701u16 => Location::Station(Station {name: StationName::Chinatown, subway_lines: &[SubwayLine::OrangeLine]}),
    // Tufts Medical Center
    6707u16 => Location::Station(Station {name: StationName::TuftsMedicalCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    6708u16 => Location::Station(Station {name: StationName::TuftsMedicalCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    6709u16 => Location::Station(Station {name: StationName::TuftsMedicalCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    6710u16 => Location::Station(Station {name: StationName::TuftsMedicalCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    6711u16 => Location::Station(Station {name: StationName::TuftsMedicalCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    2105u16 => Location::Station(Station {name: StationName::TuftsMedicalCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    7004u16 => Location::Station(Station {name: StationName::TuftsMedicalCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    1941u16 => Location::Station(Station {name: StationName::TuftsMedicalCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    7006u16 => Location::Station(Station {name: StationName::TuftsMedicalCenter, subway_lines: &[SubwayLine::OrangeLine]}),
    // Back Bay
    7007u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    1480u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    6714u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    6715u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    6716u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    6717u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    6718u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    6719u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    6720u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    1801u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    7009u16 => Location::Station(Station {name: StationName::BackBay, subway_lines: &[SubwayLine::OrangeLine]}),
    // Massachusetts Avenue
    7010u16 => Location::Station(Station {name: StationName::MassachusettsAvenue, subway_lines: &[SubwayLine::OrangeLine]}),
    2118u16 => Location::Station(Station {name: StationName::MassachusettsAvenue, subway_lines: &[SubwayLine::OrangeLine]}),
    6724u16 => Location::Station(Station {name: StationName::MassachusettsAvenue, subway_lines: &[SubwayLine::OrangeLine]}),
    6723u16 => Location::Station(Station {name: StationName::MassachusettsAvenue, subway_lines: &[SubwayLine::OrangeLine]}),
    6722u16 => Location::Station(Station {name: StationName::MassachusettsAvenue, subway_lines: &[SubwayLine::OrangeLine]}),
    6721u16 => Location::Station(Station {name: StationName::MassachusettsAvenue, subway_lines: &[SubwayLine::OrangeLine]}),
    // Ruggles
    6726u16 => Location::Station(Station {name: StationName::Ruggles, subway_lines: &[SubwayLine::OrangeLine]}),
    6727u16 => Location::Station(Station {name: StationName::Ruggles, subway_lines: &[SubwayLine::OrangeLine]}),
    6728u16 => Location::Station(Station {name: StationName::Ruggles, subway_lines: &[SubwayLine::OrangeLine]}),
    2122u16 => Location::Station(Station {name: StationName::Ruggles, subway_lines: &[SubwayLine::OrangeLine]}),
    2123u16 => Location::Station(Station {name: StationName::Ruggles, subway_lines: &[SubwayLine::OrangeLine]}),
    2124u16 => Location::Station(Station {name: StationName::Ruggles, subway_lines: &[SubwayLine::OrangeLine]}),
    1804u16 => Location::Station(Station {name: StationName::Ruggles, subway_lines: &[SubwayLine::OrangeLine]}),
    // Roxbury Crossing
    6737u16 => Location::Station(Station {name: StationName::RoxburyCrossing, subway_lines: &[SubwayLine::OrangeLine]}),
    6736u16 => Location::Station(Station {name: StationName::RoxburyCrossing, subway_lines: &[SubwayLine::OrangeLine]}),
    6735u16 => Location::Station(Station {name: StationName::RoxburyCrossing, subway_lines: &[SubwayLine::OrangeLine]}),
    6734u16 => Location::Station(Station {name: StationName::RoxburyCrossing, subway_lines: &[SubwayLine::OrangeLine]}),
    6733u16 => Location::Station(Station {name: StationName::RoxburyCrossing, subway_lines: &[SubwayLine::OrangeLine]}),
    2125u16 => Location::Station(Station {name: StationName::RoxburyCrossing, subway_lines: &[SubwayLine::OrangeLine]}),
    7012u16 => Location::Station(Station {name: StationName::RoxburyCrossing, subway_lines: &[SubwayLine::OrangeLine]}),
    // Jackson Square
    6741u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6740u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6739u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    2131u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    7013u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    7014u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    2135u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6743u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6744u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    6745u16 => Location::Station(Station {name: StationName::JacksonSquare, subway_lines: &[SubwayLine::OrangeLine]}),
    // Green Street
    6746u16 => Location::Station(Station {name: StationName::GreenStreet, subway_lines: &[SubwayLine::OrangeLine]}),
    6747u16 => Location::Station(Station {name: StationName::GreenStreet, subway_lines: &[SubwayLine::OrangeLine]}),
    6748u16 => Location::Station(Station {name: StationName::GreenStreet, subway_lines: &[SubwayLine::OrangeLine]}),
    2142u16 => Location::Station(Station {name: StationName::GreenStreet, subway_lines: &[SubwayLine::OrangeLine]}),
    7015u16 => Location::Station(Station {name: StationName::GreenStreet, subway_lines: &[SubwayLine::OrangeLine]}),
    // Forest Hills
    6750u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    6751u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    6752u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    6753u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    6754u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    6755u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    2150u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    7016u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    6950u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    6951u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    604u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    7096u16 => Location::Station(Station {name: StationName::ForestHills, subway_lines: &[SubwayLine::OrangeLine]}),
    // South Station
    7039u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    2215u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6816u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6817u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6818u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6819u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6820u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6821u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6822u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6823u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    7040u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    2228u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6827u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6826u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6825u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    6824u16 => Location::Station(Station {name: StationName::SouthStation, subway_lines: &[SubwayLine::RedLine, SubwayLine::SilverLine]}),
    // Courthouse
    6929u16 => Location::Station(Station {name: StationName::Courthouse, subway_lines: &[SubwayLine::SilverLine]}),
    2357u16 => Location::Station(Station {name: StationName::Courthouse, subway_lines: &[SubwayLine::SilverLine]}),
    7079u16 => Location::Station(Station {name: StationName::Courthouse, subway_lines: &[SubwayLine::SilverLine]}),
    6933u16 => Location::Station(Station {name: StationName::Courthouse, subway_lines: &[SubwayLine::SilverLine]}),
    6932u16 => Location::Station(Station {name: StationName::Courthouse, subway_lines: &[SubwayLine::SilverLine]}),
    2358u16 => Location::Station(Station {name: StationName::Courthouse, subway_lines: &[SubwayLine::SilverLine]}),
    6792u16 => Location::Station(Station {name: StationName::Courthouse, subway_lines: &[SubwayLine::SilverLine]}),
    // Bowdoin
    6937u16 => Location::Station(Station {name: StationName::Bowdoin, subway_lines: &[SubwayLine::BlueLine]}),
    2367u16 => Location::Station(Station {name: StationName::Bowdoin, subway_lines: &[SubwayLine::BlueLine]}),
    7085u16 => Location::Station(Station {name: StationName::Bowdoin, subway_lines: &[SubwayLine::BlueLine]}),
    // Government Center
    6963u16 => Location::Station(Station {name: StationName::GovernmentCenter, subway_lines: &[SubwayLine::BlueLine]}),
    6962u16 => Location::Station(Station {name: StationName::GovernmentCenter, subway_lines: &[SubwayLine::BlueLine]}),
    6961u16 => Location::Station(Station {name: StationName::GovernmentCenter, subway_lines: &[SubwayLine::BlueLine]}),
    6960u16 => Location::Station(Station {name: StationName::GovernmentCenter, subway_lines: &[SubwayLine::BlueLine]}),
    6959u16 => Location::Station(Station {name: StationName::GovernmentCenter, subway_lines: &[SubwayLine::BlueLine]}),
    6958u16 => Location::Station(Station {name: StationName::GovernmentCenter, subway_lines: &[SubwayLine::BlueLine]}),
    5298u16 => Location::Station(Station {name: StationName::GovernmentCenter, subway_lines: &[SubwayLine::BlueLine]}),
    // Aquarium
    6609u16 => Location::Station(Station {name: StationName::Aquarium, subway_lines: &[SubwayLine::BlueLine]}),
    6608u16 => Location::Station(Station {name: StationName::Aquarium, subway_lines: &[SubwayLine::BlueLine]}),
    1877u16 => Location::Station(Station {name: StationName::Aquarium, subway_lines: &[SubwayLine::BlueLine]}),
    6965u16 => Location::Station(Station {name: StationName::Aquarium, subway_lines: &[SubwayLine::BlueLine]}),
    6610u16 => Location::Station(Station {name: StationName::Aquarium, subway_lines: &[SubwayLine::BlueLine]}),
    1880u16 => Location::Station(Station {name: StationName::Aquarium, subway_lines: &[SubwayLine::BlueLine]}),
    1871u16 => Location::Station(Station {name: StationName::Aquarium, subway_lines: &[SubwayLine::BlueLine]}),
    6966u16 => Location::Station(Station {name: StationName::Aquarium, subway_lines: &[SubwayLine::BlueLine]}),
    // Maverick
    7088u16 => Location::Station(Station {name: StationName::Maverick, subway_lines: &[SubwayLine::BlueLine]}),
    6944u16 => Location::Station(Station {name: StationName::Maverick, subway_lines: &[SubwayLine::BlueLine]}),
    4384u16 => Location::Station(Station {name: StationName::Maverick, subway_lines: &[SubwayLine::BlueLine]}),
    6946u16 => Location::Station(Station {name: StationName::Maverick, subway_lines: &[SubwayLine::BlueLine]}),
    6947u16 => Location::Station(Station {name: StationName::Maverick, subway_lines: &[SubwayLine::BlueLine]}),
    6948u16 => Location::Station(Station {name: StationName::Maverick, subway_lines: &[SubwayLine::BlueLine]}),
    6949u16 => Location::Station(Station {name: StationName::Maverick, subway_lines: &[SubwayLine::BlueLine]}),
    1840u16 => Location::Station(Station {name: StationName::Maverick, subway_lines: &[SubwayLine::BlueLine]}),
    7083u16 => Location::Station(Station {name: StationName::Maverick, subway_lines: &[SubwayLine::BlueLine]}),
    // Airport
    6613u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    6612u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    6611u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    6968u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    2009u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    6616u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    6615u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    6614u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    6970u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    1847u16 => Location::Station(Station {name: StationName::Airport, subway_lines: &[SubwayLine::BlueLine]}),
    // Wood Island
    6618u16 => Location::Station(Station {name: StationName::WoodIsland, subway_lines: &[SubwayLine::BlueLine]}),
    6619u16 => Location::Station(Station {name: StationName::WoodIsland, subway_lines: &[SubwayLine::BlueLine]}),
    2010u16 => Location::Station(Station {name: StationName::WoodIsland, subway_lines: &[SubwayLine::BlueLine]}),
    6971u16 => Location::Station(Station {name: StationName::WoodIsland, subway_lines: &[SubwayLine::BlueLine]}),
    // Orient Heights
    6621u16 => Location::Station(Station {name: StationName::OrientHeights, subway_lines: &[SubwayLine::BlueLine]}),
    6622u16 => Location::Station(Station {name: StationName::OrientHeights, subway_lines: &[SubwayLine::BlueLine]}),
    6623u16 => Location::Station(Station {name: StationName::OrientHeights, subway_lines: &[SubwayLine::BlueLine]}),
    2014u16 => Location::Station(Station {name: StationName::OrientHeights, subway_lines: &[SubwayLine::BlueLine]}),
    6972u16 => Location::Station(Station {name: StationName::OrientHeights, subway_lines: &[SubwayLine::BlueLine]}),
    6974u16 => Location::Station(Station {name: StationName::OrientHeights, subway_lines: &[SubwayLine::BlueLine]}),
    1868u16 => Location::Station(Station {name: StationName::OrientHeights, subway_lines: &[SubwayLine::BlueLine]}),
    // Suffolk Downs
    6625u16 => Location::Station(Station {name: StationName::SuffolkDowns, subway_lines: &[SubwayLine::BlueLine]}),
    6626u16 => Location::Station(Station {name: StationName::SuffolkDowns, subway_lines: &[SubwayLine::BlueLine]}),
    2017u16 => Location::Station(Station {name: StationName::SuffolkDowns, subway_lines: &[SubwayLine::BlueLine]}),
    6975u16 => Location::Station(Station {name: StationName::SuffolkDowns, subway_lines: &[SubwayLine::BlueLine]}),
    // Beachmont
    6628u16 => Location::Station(Station {name: StationName::Beachmont, subway_lines: &[SubwayLine::BlueLine]}),
    6629u16 => Location::Station(Station {name: StationName::Beachmont, subway_lines: &[SubwayLine::BlueLine]}),
    6630u16 => Location::Station(Station {name: StationName::Beachmont, subway_lines: &[SubwayLine::BlueLine]}),
    2021u16 => Location::Station(Station {name: StationName::Beachmont, subway_lines: &[SubwayLine::BlueLine]}),
    6976u16 => Location::Station(Station {name: StationName::Beachmont, subway_lines: &[SubwayLine::BlueLine]}),
    // Revere Beach
    6632u16 => Location::Station(Station {name: StationName::RevereBeach, subway_lines: &[SubwayLine::BlueLine]}),
    6633u16 => Location::Station(Station {name: StationName::RevereBeach, subway_lines: &[SubwayLine::BlueLine]}),
    2024u16 => Location::Station(Station {name: StationName::RevereBeach, subway_lines: &[SubwayLine::BlueLine]}),
    6977u16 => Location::Station(Station {name: StationName::RevereBeach, subway_lines: &[SubwayLine::BlueLine]}),
    // Wonderland
    6638u16 => Location::Station(Station {name: StationName::Wonderland, subway_lines: &[SubwayLine::BlueLine]}),
    6637u16 => Location::Station(Station {name: StationName::Wonderland, subway_lines: &[SubwayLine::BlueLine]}),
    6636u16 => Location::Station(Station {name: StationName::Wonderland, subway_lines: &[SubwayLine::BlueLine]}),
    2025u16 => Location::Station(Station {name: StationName::Wonderland, subway_lines: &[SubwayLine::BlueLine]}),
    6978u16 => Location::Station(Station {name: StationName::Wonderland, subway_lines: &[SubwayLine::BlueLine]}),


};
