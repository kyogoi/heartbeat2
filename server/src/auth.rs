use uuid;

pub struct Password {
    pub plain: String,
    pub hash: String,
}

impl Password {
    pub fn new() -> Result<Self, bcrypt::BcryptError> {
        let password = uuid::Uuid::new_v4().to_string();

        let hash = bcrypt::hash(password.clone(), bcrypt::DEFAULT_COST)?;

        Ok(Self {
            plain: password,
            hash,
        })
    }
}

pub fn generate_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn generate_username() -> String {
    format!("device_{}", uuid::Uuid::new_v4().simple())
}
