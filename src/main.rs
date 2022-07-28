use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

type ReqResult<T> = Result<T, reqwest::Error>;

#[derive(Deserialize, Debug)]
struct UserFields {
    #[serde(deserialize_with = "parse_null_string")]
    email: String,
    #[serde(deserialize_with = "parse_null_string")]
    username: String,
    #[serde(deserialize_with = "parse_null_string")]
    bio: String,
    #[serde(deserialize_with = "parse_null_string")]
    image: String,
    #[serde(deserialize_with = "parse_null_string")]
    pub token: String,
}

#[derive(Deserialize, Debug)]
struct User {
    pub user: UserFields,
}

fn parse_null_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}

async fn authenticate(email: &str, password: &str) -> ReqResult<String> {
    let client = Client::new();
    let body = json!({
        "user": {
            "email": email,
            "password": password,
        }
    });
    let res = client
        .post("http://10.0.34.152:3000/api/users/login")
        .json(&body)
        .send()
        .await?;
    let user = res.json::<User>().await?;
    Ok(user.user.token)
}

async fn articles(token: Option<&str>) -> ReqResult<()> {
    let url = "http://10.0.34.152:3000/api/articles";
    let client = Client::new();
    let req = client.get(url);
    let req = if let Some(token) = token {
        req.bearer_auth(token)
    } else {
        req
    };
    let res = req.send().await?;
    println!("Articles: {:?}", res.text().await?);
    Ok(())
}

async fn profile(name: &str) -> ReqResult<()> {
    let url = format!("{}{}", "http://10.0.34.152:3000/api/articles/", name);
    println!("get: {:?}", reqwest::get(url).await?.text().await?);
    Ok(())
}

async fn registration(email: &str, password: &str, username: &str) -> ReqResult<String> {
    let body = json!({
        "user": {
            "username": username,
            "email": email,
            "password": password,
        }
    });
    let client = Client::new();
    let user = client
        .post("http://10.0.34.152:3000/api/users")
        .json(&body)
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(user.user.token)
}

async fn current_user(token: &str) -> ReqResult<()> {
    let client = Client::new();
    let _res = client
        .get("http://10.0.34.152:3000/api/user")
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}

async fn create_article(
    token: &str,
    title: &str,
    description: &str,
    body: &str,
    tag_list: Vec<&str>,
) -> ReqResult<()> {
    let client = Client::new();
    let body = json!({
        "article": {
            "title": title,
            "description": description,
            "body": body,
            "tagList": tag_list,
        }
    });
    let res = client
        .post("http://10.0.34.152:3000/api/articles")
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    println!("article: {:?}", res.text().await?);
    Ok(())
}

#[tokio::main]
async fn main() -> ReqResult<()> {
    //let token = registration("dan@mail.com", "pass", "Dan").await?;
    let token = authenticate("dan@mail.com", "pass").await?;
    current_user(&token).await?;

    let _res = create_article(
        &token,
        "Reqwest Tutorial",
        "How to use reqwest",
        "Today we are talking about reqwest",
        vec!["rust", "reqwest"],
    )
    .await?;

    //let token = registration("ben@mail.com", "pass", "Ben").await?;
    let token = authenticate("ben@mail.com", "pass").await?;
    current_user(&token).await?;

    let _res = create_article(
        &token,
        "Real World App",
        "How to use the Real World API",
        "Today we are talking about the Real World App",
        vec!["real world", "rust"],
    )
    .await?;

    articles(Some(&token)).await?;

    Ok(())
}
