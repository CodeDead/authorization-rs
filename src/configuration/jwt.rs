use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct JWT {
    pub secret: String,
    pub expires: i64,
}
