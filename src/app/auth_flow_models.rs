use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AbstractWSMessage {
    pub r#type: String,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Password {
    pub password: String,
    // TODO create string enum of password types: plain, rsa2, aes, 2fa, multi, signature, totp, oauth, code
    pub r#type: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetAvailabilityAuthTypesWSMessage {
    pub r#type: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthTypeDetails {
    pub r#type: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthType {
    pub details: Vec<AuthTypeDetails>,
    pub name: String,
    pub displayName: String,
    pub visible: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AvailableAuthTypes {
    pub list: Vec<AuthType>,
    pub features: u32,
    pub r#type: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthWSMessage {
    pub r#type: String,
    pub authType: String,
    pub login: String,
    pub auth_id: String,
    pub password: Password,
    pub getSession: bool,
}
