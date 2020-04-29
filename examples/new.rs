use futures::TryStreamExt;
use tiberius::{AuthMethod, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut builder = Client::builder();
    builder.host("0.0.0.0");
    builder.port(1433);
    builder.database("master");
    builder.authentication(AuthMethod::sql_server("SA", "<YourStrong@Passw0rd>"));

    let mut conn = builder.build().await?;
    let stream = conn.query("SELECT 1", &[]).await?;

    let rows: Vec<_> = stream.map_ok(|x| x.get::<_, i32>(0)).try_collect().await?;
    assert_eq!(1i32, rows[0]);
    dbg!(rows);

    Ok(())
}