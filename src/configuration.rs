use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[repr(C)]
pub struct Configuration {
    yaw_p: f32,
    yaw_i: f32,
    yaw_d: f32,

    pitch_p: f32,
    pitch_i: f32,
    pitch_d: f32,

    roll_p: f32,
    roll_i: f32,
    roll_d: f32,
}

#[cfg(test)]
mod test {
    use core::ops::Deref;
    use heapless::Vec;
    use postcard::{from_bytes, to_vec};

    use super::*;

    #[test]
    fn config_response() {
        let mut config = Configuration::default();
        config.pitch_d = core::f32::consts::E;
        let output: Vec<u8, 36> = to_vec(&config).unwrap();
        let back: Configuration = from_bytes(output.deref()).unwrap();
        assert_eq!(back.pitch_d, core::f32::consts::E);
    }

    #[test]
    fn config_size_is_36() {
        assert_eq!(36, core::mem::size_of::<Configuration>());
    }
}
