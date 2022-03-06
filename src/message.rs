use crate::parameter;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Parameter(parameter::Write),
    RequestParams,
}
