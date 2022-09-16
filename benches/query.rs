use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::{Client, Config};
use tokio_postgres::config::{SslMode, Host};

const USER: &str = "postgres";
const PASSWORD: &str = "noria";
const DB_NAME: &str = "noria";
const SSL_MODE: SslMode = SslMode::Disable;
const HOST: &str = "127.0.0.1";
const RS_PORT: u16 = 5433;
const PSQL_PORT: u16 = 5432;

async fn simple_query(client: &Client) {
    client.simple_query("SELECT * FROM t").await.unwrap();
}

async fn setup() -> Client {
    let mut config = Config::new();
    config.user(USER);
    config.password(PASSWORD);
    config.dbname(DB_NAME);
    config.host(HOST);
    config.ssl_mode(SSL_MODE);
    config.port(PSQL_PORT);

    let connector = TlsConnector::builder().build().unwrap(); // Never returns an error
    let tls = MakeTlsConnector::new(connector);
    let (client, connection) = config.connect(tls).await.unwrap();
    tokio::spawn(connection);

    client
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(setup());
    let mut g = c.benchmark_group("test");
    g.throughput(Throughput::Elements(1));
    g.bench_function("simple query", |b| b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {simple_query(&client).await}));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);