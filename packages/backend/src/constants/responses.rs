use std::fmt::Display;

use serde::{Deserialize, Serialize, Serializer};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GetServerInformationResponse {
    contributors: Vec<String>,
    contact: String,
}

impl Default for GetServerInformationResponse {
    fn default() -> Self {
        Self {
            contributors: vec!["Bhattarpong Somwong".to_string()],
            contact: "numbbutt34685@gmail.com".to_string(),
        }
    }
}

/// General response for simple operations for most `post` events.
#[derive(Serialize, ToSchema)]
pub struct DefaultSuccessResponse {
    message: String,
}

impl Default for DefaultSuccessResponse {
    fn default() -> Self {
        Self {
            message: "completed".to_string(),
        }
    }
}

impl DefaultSuccessResponse {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

/// Trait to denote a type that would be translated to `bigint` in `typescript`
pub trait Bigint
where
    Self: Display,
{
}

impl Bigint for u64 {}
impl Bigint for i64 {}
impl Bigint for u128 {}
impl Bigint for i128 {}

/// Serialize a type which would be called `bigint` in `typescript` to `string` because `bigint` is
/// technically not supported in most of `typescript` and also in `next`
///
/// todo: this might be removed when `bigint` support fully arrives in `typescript` and `next`
pub fn serialize_bigint_to_string<V, S>(value: &V, serializer: S) -> Result<S::Ok, S::Error>
where
    V: Bigint,
    S: Serializer,
{
    serializer.serialize_str(format!("{}", value).as_str())
}
