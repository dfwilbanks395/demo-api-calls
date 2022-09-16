mod article;
mod parse_utils;
mod canned_data;

use article::Article;
use parse_utils::parse_null_string;
use rand::distributions::{Alphanumeric, DistString, Distribution, Uniform};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

pub type ReqResult<T> = Result<T, reqwest::Error>;

const BASE_URL: &str = "http://localhost:3000/api/";
const NUM_TAGS: usize = 10;
const NUM_USERS: usize = 10;
const NUM_ARTICLES: usize = 10;

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

/// A struct user to authenticate users. Keeps the most recently assigned authentication token, which may change or become invalid.
struct UserAuth {
    /// User's email
    email: String,
    /// User's password
    password: String,
    /// Authentication token returned after last registration or sign in
    current_token: String,
}

/// Generates a random string given a lower and upper bound on the number of characters
fn random_string(lower: usize, upper: usize) -> String {
    let mut rng = rand::thread_rng();
    let username_size = Uniform::new(lower, upper).sample(&mut rng);
    Alphanumeric.sample_string(&mut rng, username_size)
}

/// Seeds the Real World database by adding data through the API
pub struct Initializer {
    /// Users and their authentication information.
    users_tokens: Vec<(String, UserAuth)>,
    /// The set of tags available for use on articles, in order of popularity
    tags: Vec<String>,
    /// The set of articles in order of popularity
    articles: Vec<String>,
}

impl Initializer {
    pub fn new() -> Self {
        Self {
            users_tokens: Default::default(),
            tags: Default::default(),
            articles: Default::default(),
        }
    }

    /// Generate N users
    async fn generate_users(&mut self, num_users: usize) -> ReqResult<()> {
        // First add the default ReadySet User
        let current_token = registration("ready@set.io", "password", "readyset").await?;
        self.users_tokens.push((
            "readyset".to_string(),
            UserAuth {
                email: "ready@set.io".to_string(),
                password: "password".to_string(),
                current_token,
            },
        ));
        for _ in 0..num_users {
            let name = random_string(8, 16);
            let email = format!("{}{}", &name, "@mail.com");
            let password = "password".to_string();
            if !self.users_tokens.iter().any(|(u, _t)| u == &name)
            {
                let current_token = registration(&email, &password, &name).await?;
                self.users_tokens.push((name, UserAuth {
                    email,
                    password,
                    current_token,
                }));
            }
        }
        Ok(())
    }

    /// Generate N tags
    async fn generate_tags(&mut self, num_tags: usize) -> ReqResult<()> {
        for _ in 0..num_tags {
            let tag = random_string(5, 10);
            self.tags.push(tag);
        }
        Ok(())
    }

    /// Generate articles for all users
    async fn generate_articles(&mut self) -> ReqResult<()> {
        let mut rng = rand::thread_rng();
        let tag_dist = zipf::ZipfDistribution::new(NUM_TAGS, 1.05).unwrap();
        let auth_dist = zipf::ZipfDistribution::new(NUM_USERS, 1.05).unwrap();
        
        for _ in 0..NUM_ARTICLES {
            let title = random_string(8, 16);
            let description = random_string(8, 16);
            let body = random_string(8, 16);

            // All articles get three tags selected by a zipfian distribution
            let tag_list = (0..3)
                .map(|_| self.tags[tag_dist.sample(&mut rng)-1].as_str())
                .collect::<Vec<_>>();
            // Pick a random author
            let auth = &self.users_tokens[auth_dist.sample(&mut rng)-1].1;

            self.articles.push(
                create_article(&auth.current_token, &title, &description, &body, tag_list).await?);
        }
        Ok(())
    }

    // Favorite articles for readyset 
    async fn favorite_articles(&mut self) -> ReqResult<()> {
        let rs = self.users_tokens.iter().find(|(u, _t)| u == "readyset").unwrap();
        for slug in &self.articles {
            favorite_article(&rs.1.current_token, slug).await?;
        }
        Ok(())
    }

    // Follow all users for readyst
    async fn follow_users(&mut self) -> ReqResult<()> {
        let rs = self.users_tokens.iter().find(|(u, _t)| u == "readyset").unwrap();
        for (u, _) in &self.users_tokens {
            if u != "readyset" {
                follow_user(&rs.1.current_token, u).await?;
            }
        }
        Ok(())
    }

    /// Run all generate functions
    pub async fn seed(&mut self) -> ReqResult<()> {
        println!("generating users");
        self.generate_users(NUM_USERS).await?;
        println!("generating tags");
        self.generate_tags(NUM_TAGS).await?;
        println!("generating articles");
        self.generate_articles().await?;
        println!("favoriting articles");
        self.favorite_articles().await?;
        println!("following users");
        self.follow_users().await?;
        Ok(())
    }
}

async fn _authenticate(email: &str, password: &str) -> ReqResult<String> {
    let client = Client::new();
    let body = json!({
        "user": {
            "email": email,
            "password": password,
        }
    });
    let res = client
        .post(format!("{}{}", BASE_URL, "users/login"))
        .json(&body)
        .send()
        .await?;
    let user = res.json::<User>().await?;
    Ok(user.user.token)
}

async fn _articles(token: Option<&str>) -> ReqResult<()> {
    let url = format!("{}{}", BASE_URL, "articles");
    let client = Client::new();
    let req = client.get(url);
    let req = if let Some(token) = token {
        req.bearer_auth(token)
    } else {
        req
    };
    let _res = req.send().await?;
    Ok(())
}

async fn _tags(token: Option<&str>) -> ReqResult<()> {
    let url = format!("{}{}", BASE_URL, "tags");
    let client = Client::new();
    let req = client.get(url);
    let req = if let Some(token) = token {
        req.bearer_auth(token)
    } else {
        req
    };
    let _res = req.send().await?;
    Ok(())
}

async fn _profile(name: &str) -> ReqResult<()> {
    let _url = format!("{}{}{}", BASE_URL, "articles/", name);
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
        .post(format!("{}{}", BASE_URL, "users"))
        .json(&body)
        .send()
        .await?
        .json::<User>()
        .await?;
    Ok(user.user.token)
}

async fn _current_user(token: &str) -> ReqResult<()> {
    let client = Client::new();
    let _res = client
        .get(format!("{}{}", BASE_URL, "users"))
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}

/// Creates an article for the user. Returns the article slug
async fn create_article<S>(
    token: &str,
    title: &str,
    description: &str,
    body: &str,
    tag_list: Vec<S>,
) -> ReqResult<String>
where
    S: serde::Serialize,
{
    let client = Client::new();
    let body = json!({
        "article": {
            "title": title,
            "description": description,
            "body": body,
            "tagList": tag_list,
        }
    });
    let article = client
        .post(format!("{}{}", BASE_URL, "articles"))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?
        .json::<Article>()
        .await?;

    Ok(article.article.slug)
}

/// Favorite an article for a user
async fn favorite_article(token: &str, slug: &str) -> ReqResult<()> {
    let url = format!("{}{}{}{}", BASE_URL, "articles/", slug, "/favorite");
    Client::new().post(url).bearer_auth(token).send().await?;
    Ok(())
}

/// Follow a user
async fn follow_user(token: &str, user: &str) -> ReqResult<()> {
    let url = format!("{}{}{}{}", BASE_URL, "profiles/", user, "/follow");
    Client::new().post(url).bearer_auth(token).send().await?;
    Ok(())
}
