#![allow(unused)] // while exploring, remove for prod.

use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::dbs::{Response, Session};
use surrealdb::engine::any::{connect, Any};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Connection, Surreal};
use surrealdb_start::model::{Name, Person, Record, Responsibility};

#[tokio::main()]
async fn main() -> Result<()> {
    // namespace - database
    let db = connect("mem://").await?;
    db.use_ns("my_ns").use_db("my_db").await?;

    // --- create
    // create a person
    create_task(&db).await;

    // -- merge
    merge_task(&db).await;

    // -- select all person
    select_task(&db).await;

    // create a custom query
    custom_query(&db).await

}

// create a new person with a random id
async fn create_task(db: &Surreal<Any>) -> Result<()> {
    let created: Vec<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Tobie",
                last: "morgan Hitchcock",
            },
            marketing: true,
        })
        .await?;
    dbg!(created);

    return Ok(());
}

// update a person record with a specific id
async fn merge_task(db: &Surreal<Any>) -> Result<()> {
    // update
    let updated: Option<Record> = db
        .update(("person", "jaime"))
        .merge(Responsibility { marketing: true })
        .await?;
    dbg!(updated);

    return Ok(());
}

// select all people records
async fn select_task(db: &Surreal<Any>) -> Result<()> {

    let peoples: Vec<Record> = db.select("person").await?;
    dbg!(peoples);

    return Ok(())
}

// custom a advanced query
async fn custom_query(db: &Surreal<Any>) -> Result<()> {
    let groups = db 
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(groups);

    return Ok(())
}


