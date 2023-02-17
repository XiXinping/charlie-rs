use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString, FromRepr};
use time::Date;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Pass {
    start_validity: Date,
    pass_type: TicketType,
}

#[derive(
    AsRefStr, Clone, Copy, Debug, Display, EnumIter, EnumString, Eq, FromRepr, PartialEq, Serialize,
)]
pub enum TicketType {
    Nothing,

    // regular card types
    Adult = 367,
    #[strum(serialize = "SV Adult")]
    SvAdult = 366,
    Student = 418,
    Senior = 419,
    #[strum(serialize = "TAP")]
    Tap = 420,
    Blind = 417,
    Child = 426,
    Employee = 414,
    Retiree = 415,
    PoliceFire = 416,

    // passes
    #[strum(serialize = "30 Day Local Bus Pass")]
    ThirtyDayLocalBusPass = 135,
    #[strum(serialize = "30 Day Inner Express Bus Pass")]
    ThirtyDayInnerExpressBusPass,
    #[strum(serialize = "30 Day Outer Express Bus Pass")]
    ThirtyDayOuterExpressBusPass,

    #[strum(serialize = "30 Day LinkPass")]
    ThirtyDayLinkPass,
    #[strum(serialize = "30 Day Senior LinkPass")]
    ThirtyDayLinkPassSenior,
    #[strum(serialize = "30 Day TAP LinkPass")]
    ThirtyDayLinkPassTap = 148,
    #[strum(serialize = "Monthly Student LinkPass")]
    MonthlyStudentLinkPass = 150,
    #[strum(serialize = "Monthly TAP LinkPass")]
    MonthlyTapLinkPass = 0b0110_1010_00,
    #[strum(serialize = "Monthly Senior LinkPass")]
    MonthlySeniorLinkPass = 0b0110_1010_01,

    #[strum(serialize = "Senior TAP/Permit")]
    SeniorTapPermit = 0b0110_1001_01,
    #[strum(serialize = "Senior TAP/Permit 30 Days")]
    SeniorTapPermit30Days = 0b0110_1001_10,

    // commuter rail passes
    #[strum(serialize = "30 Day Commuter Rail Zone 1A Pass")]
    ThirtyDayZone1APass = 166,
    #[strum(serialize = "30 Day Commuter Rail Zone 1 Pass")]
    ThirtyDayZone1Pass,
    #[strum(serialize = "30 Day Commuter Rail Zone 2 Pass")]
    ThirtyDayZone2Pass,
    #[strum(serialize = "30 Day Commuter Rail Zone 3 Pass")]
    ThirtyDayZone3Pass,
    #[strum(serialize = "30 Day Commuter Rail Zone 4 Pass")]
    ThirtyDayZone4Pass,
    #[strum(serialize = "30 Day Commuter Rail Zone 5 Pass")]
    ThirtyDayZone5Pass,
    #[strum(serialize = "30 Day Commuter Rail Zone 6 Pass")]
    ThirtyDayZone6Pass,
    #[strum(serialize = "30 Day Commuter Rail Zone 7 Pass")]
    ThirtyDayZone7Pass,
    #[strum(serialize = "30 Day Commuter Rail Zone 8 Pass")]
    ThirtyDayZone8Pass,

    #[strum(serialize = "30 Day Interzone 1 Pass")]
    ThirtyDayInterzone1Pass,
    #[strum(serialize = "30 Day Interzone 2 Pass")]
    ThirtyDayInterzone2Pass,
    #[strum(serialize = "30 Day Interzone 3 Pass")]
    ThirtyDayInterzone3Pass,
    #[strum(serialize = "30 Day Interzone 4 Pass")]
    ThirtyDayInterzone4Pass,
    #[strum(serialize = "30 Day Interzone 5 Pass")]
    ThirtyDayInterzone5Pass,
    #[strum(serialize = "30 Day Interzone 6 Pass")]
    ThirtyDayInterzone6Pass,
    #[strum(serialize = "30 Day Interzone 7 Pass")]
    ThirtyDayInterzone7Pass,
    #[strum(serialize = "30 Day Interzone 8 Pass")]
    ThirtyDayInterzone8Pass,

    #[strum(serialize = "10 Ride Zone 1A")]
    TenRideZone1A = 355,
    #[strum(serialize = "12 Ride Zone 1B")]
    TenRideZone1B,
    #[strum(serialize = "10 Ride Zone 1")]
    TenRideZone1,
    #[strum(serialize = "10 Ride Zone 2")]
    TenRideZone2,
    #[strum(serialize = "10 Ride Zone 3")]
    TenRideZone3,
    #[strum(serialize = "10 Ride Zone 4")]
    TenRideZone4,
    #[strum(serialize = "10 Ride Zone 5")]
    TenRideZone5,
    #[strum(serialize = "10 Ride Zone 6")]
    TenRideZone6,
    #[strum(serialize = "10 Ride Zone 7")]
    TenRideZone7,
    #[strum(serialize = "10 Ride Zone 8")]
    TenRideZone8,

    #[strum(serialize = "One Way Interzone Adult 1 Zone")]
    OneWayInterzoneAdult1Zone = 140,
    #[strum(serialize = "One Way Interzone Adult 2 Zones")]
    OneWayInterzoneAdult2Zones,
    #[strum(serialize = "One Way Interzone Adult 3 Zones")]
    OneWayInterzoneAdult3Zones,
    #[strum(serialize = "One Way Interzone Adult 4 Zones")]
    OneWayInterzoneAdult4Zones,
    #[strum(serialize = "One Way Interzone Adult 5 Zones")]
    OneWayInterzoneAdult5Zones,
    #[strum(serialize = "One Way Interzone Adult 6 Zones")]
    OneWayInterzoneAdult6Zones,
    #[strum(serialize = "One Way Interzone Adult 7 Zones")]
    OneWayInterzoneAdult7Zones,
    #[strum(serialize = "One Way Interzone Adult 8 Zones")]
    OneWayInterzoneAdult8Zones,

    #[strum(serialize = "One Way Half Fare Zone 1")]
    OneWayHalfFareZone1 = 428,
    #[strum(serialize = "One Way Half Fare Zone 2")]
    OneWayHalfFareZone2,
    #[strum(serialize = "One Way Half Fare Zone 3")]
    OneWayHalfFareZone3,
    #[strum(serialize = "One Way Half Fare Zone 4")]
    OneWayHalfFareZone4,
    #[strum(serialize = "One Way Half Fare Zone 5")]
    OneWayHalfFareZone5,
    #[strum(serialize = "One Way Half Fare Zone 6")]
    OneWayHalfFareZone6,
    #[strum(serialize = "One Way Half Fare Zone 7")]
    OneWayHalfFareZone7,
    #[strum(serialize = "One Way Half Fare Zone 8")]
    OneWayHalfFareZone8,

    #[strum(serialize = "One Way Interzone Half Fare 1 Zone")]
    OneWayInterzoneHalfFare1Zone = 436,
    #[strum(serialize = "One Way Interzone Half Fare 2 Zones")]
    OneWayInterzoneHalfFare2Zones,
    #[strum(serialize = "One Way Interzone Half Fare 3 Zones")]
    OneWayInterzoneHalfFare3Zones,
    #[strum(serialize = "One Way Interzone Half Fare 4 Zones")]
    OneWayInterzoneHalfFare4Zones,
    #[strum(serialize = "One Way Interzone Half Fare 5 Zones")]
    OneWayInterzoneHalfFare5Zones,
    #[strum(serialize = "One Way Interzone Half Fare 6 Zones")]
    OneWayInterzoneHalfFare6Zones,
    #[strum(serialize = "One Way Interzone Half Fare 7 Zones")]
    OneWayInterzoneHalfFare7Zones,
    #[strum(serialize = "One Way Interzone Half Fare 8 Zones")]
    OneWayInterzoneHalfFare8Zones,

    #[strum(serialize = "Group Interzone 1 Zones")]
    GroupInterzone1Zones = 509,
    #[strum(serialize = "Group Interzone 2 Zones")]
    GroupInterzone2Zones,
    #[strum(serialize = "Group Interzone 3 Zones")]
    GroupInterzone3Zones,
    #[strum(serialize = "Group Interzone 4 Zones")]
    GroupInterzone4Zones,
    #[strum(serialize = "Group Interzone 5 Zones")]
    GroupInterzone5Zones,
    #[strum(serialize = "Group Interzone 6 Zones")]
    GroupInterzone6Zones,
    #[strum(serialize = "Group Interzone 7 Zones")]
    GroupInterzone7Zones,
    #[strum(serialize = "Group Interzone 8 Zones")]
    GroupInterzone8Zones,

    #[strum(serialize = "Zone 1 Student Monthly Pass")]
    Zone1MonthlyPassStudent = 952,
    #[strum(serialize = "Zone 2 Student Monthly Pass")]
    Zone2MonthlyPassStudent,
    #[strum(serialize = "Zone 3 Student Monthly Pass")]
    Zone3MonthlyPassStudent,
    #[strum(serialize = "Zone 4 Student Monthly Pass")]
    Zone4MonthlyPassStudent,
    #[strum(serialize = "Zone 5 Student Monthly Pass")]
    Zone5MonthlyPassStudent,
    #[strum(serialize = "Zone 6 Student Monthly Pass")]
    Zone6MonthlyPassStudent,
    #[strum(serialize = "Zone 7 Student Monthly Pass")]
    Zone7MonthlyPassStudent,
    #[strum(serialize = "Zone 8 Student Monthly Pass")]
    Zone8MonthlyPassStudent,
    #[strum(serialize = "Zone 9 Student Monthly Pass")]
    Zone9MonthlyPassStudent,
    #[strum(serialize = "Zone 10 Student Monthly Pass")]
    Zone10MonthlyPassStudent,

    #[strum(serialize = "Interzone 1 Zone Student Monthly Pass")]
    Interzone1ZoneMonthlyPassStudent = 963,
    #[strum(serialize = "Interzone 2 Zones Student Monthly Pass")]
    Interzone2ZonesMonthlyPassStudent,
    #[strum(serialize = "Interzone 3 Zones Student Monthly Pass")]
    Interzone3ZonesMonthlyPassStudent,
    #[strum(serialize = "Interzone 4 Zones Student Monthly Pass")]
    Interzone4ZonesMonthlyPassStudent,
    #[strum(serialize = "Interzone 5 Zones Student Monthly Pass")]
    Interzone5ZonesMonthlyPassStudent,
    #[strum(serialize = "Interzone 6 Zones Student Monthly Pass")]
    Interzone6ZonesMonthlyPassStudent,
    #[strum(serialize = "Interzone 7 Zones Student Monthly Pass")]
    Interzone7ZonesMonthlyPassStudent,
    #[strum(serialize = "Interzone 8 Zones Student Monthly Pass")]
    Interzone8ZonesMonthlyPassStudent,
    #[strum(serialize = "Interzone 9 Zones Student Monthly Pass")]
    Interzone9ZonesMonthlyPassStudent,
    #[strum(serialize = "Interzone 10 Zones Student Monthly Pass")]
    Interzone10ZonesMonthlyPassStudent,
}
