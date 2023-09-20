use std::process;
use futures::executor::block_on;
use sea_orm::*;
use caesar::entities::{prelude::*,*};
use caesar::entities::bakery::ActiveModel;

const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432/";
const DB_NAME: &str = "caesar";

async fn _check() -> Result<(), DbErr> {
    Database::connect(DATABASE_URL).await?;
    Ok(())
}

async fn insert_bakery(db: &DatabaseConnection, name: &str) -> Result<InsertResult<ActiveModel>, DbErr>{
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(db).await?;

    Ok(res)
}

async fn run() -> Result<(), DbErr> {
    let db: DatabaseConnection = Database::connect(DATABASE_URL.to_owned() + DB_NAME).await?;
    let result = insert_bakery(&db, "Happy Bakery").await?;
    println!("bakery_res: {:?}", result);

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
