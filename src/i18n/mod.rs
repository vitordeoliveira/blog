// mod translation;
use serde::Deserialize;
// Flatten
// pub use translation::*;

use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::request::Parts,
    RequestPartsExt,
};
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::error::ServerError;

#[derive(Clone, EnumString, EnumIter, Display, PartialEq, Debug, Default, Deserialize)]
// #[strum(serialize_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum I18N {
    #[default]
    En,
    Pt,
}

impl I18N {
    pub fn langs(&self) -> I18NIter {
        I18N::iter()
    }
}

#[derive(Deserialize, Debug)]
struct Params {
    i18n: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for I18N
where
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // let path = parts.uri.path().to_string();

        let lang_segment = parts
            .extract::<Path<Params>>()
            .await
            .map(|path| path.i18n.clone())
            .unwrap_or_default();

        Ok(I18N::from_str(&lang_segment).unwrap_or_default())
    }
}
