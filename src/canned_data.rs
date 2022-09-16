use crate::{registration, create_article, favorite_article, follow_user, ReqResult};

async fn generate_data() -> ReqResult<()> {
    //let mut init = Initializer::new();
    //init.seed().await?;
    let t1 = registration("alice@mail.com", "password", "alice").await?;
    let t2 = registration("bob@mail.com", "password", "bob").await?;
    let t3 = registration("charles@mail.com", "password", "charles").await?;
    let t4 = registration("readyset@mail.com", "password", "readyset").await?;

    let s1 = create_article(
        &t1,
        "Alice's first article",
        "Alice",
        "This is the article",
        vec!["alice", "prisma", "readyset"],
    )
    .await?;

    let s2 = create_article(
        &t2,
        "Bob's first article",
        "Bob",
        "This is the article",
        vec!["Bob", "prisma", "readyset"],
    )
    .await?;

    let s3 = create_article(
        &t3,
        "Charles' first article",
        "Charles",
        "This is the article",
        vec!["charles", "prisma", "readyset"],
    )
    .await?;

    let s4 = create_article(
        &t4,
        "ReadySet's first article",
        "ReadySet",
        "This is the article",
        vec!["prisma", "readyset"],
    )
    .await?;

    favorite_article(&t4, &s1).await?;
    favorite_article(&t4, &s2).await?;
    favorite_article(&t4, &s3).await?;

    follow_user(&t4, "alice").await?;
    follow_user(&t4, "bob").await?;
    follow_user(&t4, "charles").await?;

    Ok(())
}