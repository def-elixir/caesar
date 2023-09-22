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

async fn update_bakery(db: &DatabaseConnection, bakery: &bakery::ActiveModel, name: &str) -> Result<bakery::Model, DbErr> {
    let mut bakery = bakery.clone();
    bakery.name = Set(name.to_owned());
    let res = bakery.update(db).await?;

    Ok(res)
}

async fn delete_bakery(db: &DatabaseConnection, bakery: &bakery::ActiveModel) -> Result<DeleteResult, DbErr> {
    let res = bakery.clone().delete(db).await?;

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
    let bakery: Option<bakery::Model> = Bakery::find()
    .filter(bakery::Column::Name.eq(name))
    .one(db)
    .await?;

    Ok(bakery)
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

    // if let Some(bakery) = find_bakery_by_id(&db, 114).await? {
    //     let update_bakery = update_bakery_with_active_model(&db, &bakery.into_active_model(), "Grate Happy").await?;
    //     println!("{:?}", update_bakery);
    // }
    // match find_bakery_by_id(&db, 2).await? {
    //     None => {
    //         println!("Not found");
    //     },
    //     Some(bakery) => {
    //         let update_bakery = update_bakery_with_active_model(&db, &bakery.into_active_model(), "Grate Happy").await?;
    //         println!("{:?}", update_bakery);
    //     }
    // }

    // match find_bakery_by_id(&db, 2).await? {
    //     None => {
    //         println!("Not found");
    //     },
    //     Some(bakery) => {
    //         let result = delete_bakery(&db, &bakery.into_active_model()).await?;
    //         println!("{:?}", result);
    //     }
    // }

    let db: DatabaseConnection = Database::connect(DATABASE_URL.to_owned() + DB_NAME).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
