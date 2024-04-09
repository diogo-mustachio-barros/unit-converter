use std::str::FromStr;

use crate::{converter::Convert, units::{UnitType, VolumeUnit}, util::enum_to_vec};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn get_unit_types() -> Vec<String> {
    enum_to_vec::<UnitType>()
}

#[tauri::command]
pub fn get_units(unit_type:String) -> Vec<String> {
    match unit_type.as_str() {
        "Volume" => enum_to_vec::<VolumeUnit>(),
        "Mass"   => vec![],
        "Length" => vec![],
        "Time"   => vec![],
        _ => vec![]
    }
}

#[tauri::command]
pub fn convert(unit_type:String, amount:f64, from_unit:String, to_unit:String) -> f64 {
    match unit_type.as_str() {
        "Volume" => {
            let from = VolumeUnit::from_str(&from_unit);
            if from.is_err() { return amount }

            let to = VolumeUnit::from_str(&to_unit);
            if to.is_err() { return amount }
            
            return VolumeUnit::convert_to(amount, from.unwrap(), to.unwrap())
        },
        "Mass"   => amount,
        "Length" => amount,
        "Time"   => amount,
        _ => amount
    }
}