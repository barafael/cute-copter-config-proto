use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[repr(packed, C)]
pub struct Config {
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

/*impl Default for Config {
    fn default() -> Self {
        Config {
            yaw_p: 0.0,
            yaw_i: 0.1,
            yaw_d: 0.2,

            pitch_p: 0.3,
            pitch_i: 0.4,
            pitch_d: 0.5,

            roll_p: 0.6,
            roll_i: 0.7,
            roll_d: 0.8,
        }
    }
}
*/
