use serde::Deserialize;

use crate::parse_utils::{parse_null_string, parse_null_string_vec};

#[derive(Deserialize, Debug)]
struct ArticleUser {
    #[serde(deserialize_with = "parse_null_string")]
    username: String,
    #[serde(deserialize_with = "parse_null_string")]
    bio: String,
    #[serde(deserialize_with = "parse_null_string")]
    image: String,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub article: ArticleFields,
}

#[derive(Deserialize, Debug)]
pub struct ArticleFields {
    #[serde(deserialize_with = "parse_null_string")]
    pub slug: String,
    #[serde(deserialize_with = "parse_null_string")]
    title: String,
    #[serde(deserialize_with = "parse_null_string")]
    description: String,
    #[serde(deserialize_with = "parse_null_string")]
    body: String,
    #[serde(deserialize_with = "parse_null_string")]
    createdAt: String,
    #[serde(deserialize_with = "parse_null_string")]
    updatedAt: String,
    #[serde(deserialize_with = "parse_null_string_vec")]
    tagList: Vec<String>,
    author: ArticleUser,
    #[serde(deserialize_with = "parse_null_string_vec")]
    favoritedBy: Vec<String>,
    favoritesCount: usize,
    favorited: bool,
}
