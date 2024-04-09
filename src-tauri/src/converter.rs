use crate::units::VolumeUnit;

pub trait Convert {
    fn convert_to(amount: f64, from: Self, to: Self) -> f64;
}

impl Convert for VolumeUnit {
    fn convert_to(amount: f64, from: Self, to: Self) -> f64 {
        let ratio_to_master = get_volume_ratio(from);
        let ratio_to_target = get_volume_ratio(to);
        
        return amount / ratio_to_target * ratio_to_master;
    }
}

fn get_volume_ratio(unit : VolumeUnit) -> f64 {
    match unit {
        // Liter
        VolumeUnit::Liter           =>   1.0,
        VolumeUnit::DeciLiter       =>   0.1,
        VolumeUnit::CentiLiter      =>   0.01,
        VolumeUnit::MilliLiter      =>   0.001,
        // Cubic meter
        VolumeUnit::CubicMeter      => 100.0,
        VolumeUnit::CubicDecimeter  =>   1.0,
        VolumeUnit::CubicCentimeter =>   0.001,
        VolumeUnit::CubicMillimeter =>   0.000001,
        // Others
        VolumeUnit::Gallon          =>   3.78541178,
    }
} 