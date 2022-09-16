struct Article {
    article: ArticleFields,
}

struct ArticleFields {
    #[serde(deserialize_with = "parse_null_string")]
    slug: String,
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
    #[serde(deserialize_with = "parse_null_string")]
    tagList: String,
    #[serde(deserialize_with = "parse_null_string")]
    author: String,
    #[serde(deserialize_with = "parse_null_string")]
    bio: String,
    #[serde(deserialize_with = "parse_null_string")]
    image: String,
    #[serde(deserialize_with = "parse_null_string")]
    favoritedBy: Vec<String>,
    #[serde(deserialize_with = "parse_null_string")]
    favoritesCount: usize,
    #[serde(deserialize_with = "parse_null_string")]
    favorited: bool,
}
