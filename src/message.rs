use crate::{command, configuration::Configuration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Configuration(command::SetParameter),
    Interactive(command::Interactive),
    RequestParams,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    Params(Configuration),
}
