use std::{fmt::Display, str::FromStr};
use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(EnumIter, Deserialize)]
pub enum UnitType {
    Volume,
    Mass,
    Length,
    Time
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Volume => write!(f, "Volume"),
            Self::Mass   => write!(f, "Mass"),
            Self::Length => write!(f, "Length"),
            Self::Time   => write!(f, "Time")
        }
    }
}

impl FromStr for UnitType {
    type Err = UnitParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Volume" => Ok(UnitType::Volume),
            "Mass"   => Ok(UnitType::Mass),
            "Length" => Ok(UnitType::Length),
            "Time"   => Ok(UnitType::Time),
            _ => Err(UnitParseErr)
        }
    }
}

#[derive(EnumIter, PartialEq, Eq, Hash)]
pub enum VolumeUnit {
    // Liter
    Liter,
    DeciLiter,
    CentiLiter,
    MilliLiter,
    // Cubic meter
    CubicMeter,
    CubicDecimeter,
    CubicCentimeter,
    CubicMillimeter,
    // Others
    Gallon,
}

impl Display for VolumeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Liter
            VolumeUnit::Liter           => write!(f, "L"),
            VolumeUnit::DeciLiter       => write!(f, "dL"),
            VolumeUnit::CentiLiter      => write!(f, "cL"),
            VolumeUnit::MilliLiter      => write!(f, "mL"),
            // Cubic meter
            VolumeUnit::CubicMeter      => write!(f, "m3"),
            VolumeUnit::CubicDecimeter  => write!(f, "dm3"),
            VolumeUnit::CubicCentimeter => write!(f, "cm3"),
            VolumeUnit::CubicMillimeter => write!(f, "mm3"),
            // Others
            VolumeUnit::Gallon          => write!(f, "gal"),
        }
    }
}

impl FromStr for VolumeUnit {
    type Err = UnitParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Liter
            "L"   => Ok(VolumeUnit::Liter),
            "dL"  => Ok(VolumeUnit::DeciLiter),
            "cL"  => Ok(VolumeUnit::CentiLiter),
            "mL"  => Ok(VolumeUnit::MilliLiter),
            // Cubic meter
            "m3"  => Ok(VolumeUnit::CubicMeter),
            "dm3" => Ok(VolumeUnit::CubicDecimeter),
            "cm3" => Ok(VolumeUnit::CubicCentimeter),
            "mm3" => Ok(VolumeUnit::CubicMillimeter),
            // Others
            "gal" => Ok(VolumeUnit::Gallon),
            _ => Err(UnitParseErr)
        }
    }
}

#[derive(Debug)]
pub struct UnitParseErr;