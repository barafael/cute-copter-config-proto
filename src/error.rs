use snafu::prelude::*;

use crate::command::SetParameter;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to write item {parameter:?}"))]
    Write { parameter: SetParameter },
}
