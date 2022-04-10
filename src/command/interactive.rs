use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Interactive {
    pub throttle: u16,
    pub roll: u16,
    pub pitch: u16,
    pub yaw: u16,
}

#[cfg(test)]
mod test {
    use crate::command::interactive::Interactive;
    use core::ops::Deref;
    use heapless::Vec;
    use postcard::{from_bytes, to_vec};

    #[test]
    fn interactive_frame() {
        let interactive = Interactive {
            throttle: 0,
            roll: 1,
            pitch: 2,
            yaw: 3,
        };
        let output: Vec<u8, 8> = to_vec(&interactive).unwrap();
        let back: Interactive = from_bytes(output.deref()).unwrap();
        assert_eq!(
            back,
            Interactive {
                throttle: 0,
                roll: 1,
                pitch: 2,
                yaw: 3,
            }
        );
    }
}
