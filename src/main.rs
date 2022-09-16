use requests::Initializer;
use requests::ReqResult;

#[tokio::main]
async fn main() -> ReqResult<()> {
    let mut init = Initializer::new();
    init.seed().await
}
