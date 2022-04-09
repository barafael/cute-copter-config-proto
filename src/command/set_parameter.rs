use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SetParameter {
    YawProportional(f32),
    YawIntegral(f32),
    YawDerivative(f32),

    PitchProportional(f32),
    PitchIntegral(f32),
    PitchDerivative(f32),

    RollProportional(f32),
    RollIntegral(f32),
    RollDerivative(f32),
}

#[cfg(test)]
mod test {
    use crate::command::SetParameter;
    use core::ops::Deref;
    use heapless::Vec;
    use postcard::{from_bytes, to_vec};

    #[test]
    fn configuration_frame() {
        let frame = SetParameter::RollDerivative(2.0);
        let output: Vec<u8, 5> = to_vec(&frame).unwrap();
        let back: SetParameter = from_bytes(output.deref()).unwrap();
        assert!(matches!(back, SetParameter::RollDerivative(x) if x == 2.0));
    }
}
