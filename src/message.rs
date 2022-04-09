use crate::{command, configuration};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Configuration(command::SetParameter),
    Interactive(command::Interactive),
    RequestParams,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Response {
    Params(configuration::Configuration),
}
