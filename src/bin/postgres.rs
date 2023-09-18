use futures::executor::block_on;
use sea_orm::{Database, DbErr};

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432";
const DB_NAME: &str = "caesar";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    println!("{:?}",db);
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
