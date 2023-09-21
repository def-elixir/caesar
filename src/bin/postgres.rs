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

async fn insert_bakery(db: &DatabaseConnection, name: &str) -> Result<InsertResult<ActiveModel>, DbErr> {
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set(name.to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(db).await?;

    Ok(res)
}

async fn select_all_bakeries(db: &DatabaseConnection) -> Result<Vec<bakery::Model>, DbErr> {
    let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;

    Ok(bakeries)
}

async fn find_bakery_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<bakery::Model>, DbErr> {
    let bakery: Option<bakery::Model> = Bakery::find_by_id(id).one(db).await?;

    Ok(bakery)
}

async fn filter_bakery(db: &DatabaseConnection, name: &str) -> Result<Option<bakery::Model>, DbErr> {
    let filter_bakery: Option<bakery::Model> = Bakery::find()
    .filter(bakery::Column::Name.eq(name))
    .one(db)
    .await?;

    Ok(filter_bakery)
}

async fn run() -> Result<(), DbErr> {
    // let result = insert_bakery(&db, "Happy Bakery").await?;
    // println!("bakery_res: {:?}", result);

    // select_all_bakeries(&db).await?.iter().for_each(|item| println!("{:?}", item));

    // match find_bakery_by_id(&db, 114514).await? {
    //     None => {
    //         println!("Not found");
    //     },
    //     Some(item) => {
    //         println!("{:?}", item);
    //     }
    // }

    // match filter_bakery(&db, "Happy Bakery").await? {
    //     None => {
    //         println!("Not found");
    //     },
    //     Some(item) => {
    //         println!("{:?}", item);
    //     }
    // }

    let db: DatabaseConnection = Database::connect(DATABASE_URL.to_owned() + DB_NAME).await?;
    // todo UPDATE & DELETE
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
