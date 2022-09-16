pub fn parse_null_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    serde::Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_else(|| "".to_string()))
}

pub fn parse_null_string_vec<'de, D>(d: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    serde::Deserialize::deserialize(d).map(|v: Vec<_>| {
        v.into_iter()
            .map(|x: Option<_>| x.unwrap_or_else(|| "".to_string()))
            .collect()
    })
}
