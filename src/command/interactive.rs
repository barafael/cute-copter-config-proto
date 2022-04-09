use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Interactive {
    Throttle(f32),
    Roll(f32),
    Pitch(f32),
    Yaw(f32),
}

#[cfg(test)]
mod test {
    use crate::command::interactive::Interactive;
    use core::ops::Deref;
    use heapless::Vec;
    use postcard::{from_bytes, to_vec};

    #[test]
    fn interactive_frame() {
        let interactive = Interactive::Roll(2.0);
        let output: Vec<u8, 5> = to_vec(&interactive).unwrap();
        let back: Interactive = from_bytes(output.deref()).unwrap();
        assert!(matches!(back, Interactive::Roll(x) if x == 2.0));
    }
}
