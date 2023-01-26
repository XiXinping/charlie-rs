use serde::Serialize;
use strum_macros::FromRepr;

#[derive(Clone, Copy, Debug, Eq, FromRepr, PartialEq, Serialize)]
pub enum CardType {
    Nothing,
    Adult = 0b0101_1011_11,
    SvAdult = 0b0101_1011_10,
    Student = 0b0110_1000_10,
    Senior = 0b0110_1000_11,
    Tap = 0b0110_1001_00,
    Blind = 0b0110_1000_01,
    Child = 0b0110_1010_10,
    Retiree = 0b0110_0111_11,
    Employee = 0b0110_0111_10,
    PoliceFire = 0b0110_1000_00,
}

#[derive(Clone, Copy, Debug, Eq, FromRepr, PartialEq, Serialize)]
pub enum PassType {
    Nothing,
    ThirtyDayLocalBusPass = 0b0010_0001_11,
    ThirtyDayInnerExpressBusPass = 0b0010_0010_00,
    ThirtyDayOuterExpressBusPass = 0b0010_0010_01,
    ThirtyDayLinkPass = 0b0010_0010_10,
    ThirtyDayLinkPassSenior = 0b0010_0010_11,
    ThirtyDayLinkPassTap = 0b0010_0101_00,
    MonthlyLinkPassStudent = 0b0010_0101_10,
    MonthlyLinkTapPass = 0b0110_1010_00,
    MonthlyLinkSeniorPass = 0b0110_1010_01,
    SeniorTapPermit = 0b0110_1001_01,
    SeniorTapPermit30Days = 0b0110_1001_10,
}
