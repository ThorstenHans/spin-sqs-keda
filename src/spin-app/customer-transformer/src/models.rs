use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InboundCustomerModel {
    #[serde(rename = "lastName")]
    pub(crate) last_name: String,
    #[serde(rename = "firstName")]
    pub(crate) first_name: String,
    pub(crate) age: u8,
}

#[derive(Serialize)]
pub struct OutboundCustomerModel {
    #[serde(rename = "lastName")]
    pub(crate) last_name: String,
    #[serde(rename = "firstName")]
    pub(crate) first_name: String,
    #[serde(rename = "fullName")]
    pub(crate) full_name: String,
    pub(crate) adult: bool,
}

impl From<&InboundCustomerModel> for OutboundCustomerModel {
    fn from(value: &InboundCustomerModel) -> Self {
        OutboundCustomerModel {
            first_name: value.first_name.clone(),
            last_name: value.last_name.clone(),
            full_name: format!("{}, {}", value.last_name, value.first_name),
            adult: value.age >= 18,
        }
    }
}
