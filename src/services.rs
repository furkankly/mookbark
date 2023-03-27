use sea_orm::*;

use crate::entities::{prelude::*, *};

use sea_orm::TransactionTrait;

pub async fn add_bookmark(
    txn: &DatabaseTransaction,
    container_name: &str,
    url: &str,
) -> Result<(), DbErr> {
    let bookmark = bookmark::ActiveModel {
        url: ActiveValue::Set(url.to_owned()),
        ..Default::default()
    };
    Bookmark::insert(bookmark).exec(txn).await?;

    txn.execute(Statement::from_sql_and_values(
        DbBackend::MySql,
        r#"
            INSERT INTO closure (ancestor, descendant)
            SELECT ancestor, ?
            FROM closure
            WHERE descendant = ?;
        "#,
        vec![url.into(), container_name.into()],
    ))
    .await?;

    txn.execute(Statement::from_sql_and_values(
        DbBackend::MySql,
        r#"
            INSERT INTO closure (ancestor, descendant)
            SELECT ?, ?;
        "#,
        vec![url.into(), url.into()],
    ))
    .await?;

    return Ok(());
}

pub async fn add_container(
    txn: &DatabaseTransaction,
    parent_container_name: &str,
    container_name: &str,
) -> Result<(), DbErr> {
    let container = container::ActiveModel {
        name: ActiveValue::Set(container_name.to_owned()),
        ..Default::default()
    };
    Container::insert(container).exec(txn).await?;

    if container_name == "root" {
        txn.execute(Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"
                INSERT INTO closure (ancestor, descendant)
                SELECT ?, ?;
            "#,
            // parent_container_name = "root" , container_name = "root"
            vec![parent_container_name.into(), container_name.into()],
        ))
        .await?;
    } else {
        txn.execute(Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"
            INSERT INTO closure (ancestor, descendant)
            SELECT ancestor, ?
            FROM closure
            WHERE descendant = ?;
        "#,
            vec![container_name.into(), parent_container_name.into()],
        ))
        .await?;

        txn.execute(Statement::from_sql_and_values(
            DbBackend::MySql,
            r#"
            INSERT INTO closure (ancestor, descendant)
            SELECT ?, ?;
        "#,
            vec![container_name.into(), container_name.into()],
        ))
        .await?;
    }

    return Ok(());
}

pub async fn add_root(db: &DatabaseConnection) -> Result<(), DbErr> {
    let txn = db.begin().await?;
    add_container(&txn, "root", "root").await?;

    txn.commit().await?;
    return Ok(());
}

pub async fn delete_entity(db: &DatabaseConnection, id: &str) -> Result<(), DbErr> {
    let txn = db.begin().await?;

    // Try deleting from bookmark in case its a bookmark
    txn.execute(Statement::from_sql_and_values(
        DbBackend::MySql,
        r#"
            DELETE
            FROM bookmark 
            WHERE url IN
                (SELECT descendant
                 FROM closure
                 WHERE descendant IN
                    (SELECT descendant
                     FROM closure
                     WHERE ancestor=?));
        "#,
        vec![id.into()],
    ))
    .await?;

    // Try deleting from container in case its a bookmark
    txn.execute(Statement::from_sql_and_values(
        DbBackend::MySql,
        r#"
            DELETE
            FROM container 
            WHERE name IN
                (SELECT descendant
                 FROM closure
                 WHERE descendant IN
                    (SELECT descendant
                     FROM closure
                     WHERE ancestor=?));
        "#,
        vec![id.into()],
    ))
    .await?;

    // Delete the path(s)
    txn.execute(Statement::from_sql_and_values(
        DbBackend::MySql,
        r#"
            DELETE
            FROM closure 
            WHERE descendant IN
                (SELECT descendant
                 FROM
                    (SELECT descendant
                     FROM closure
                     WHERE ancestor=?)
                 AS d);
        "#,
        vec![id.into()],
    ))
    .await?;

    txn.commit().await?;
    Ok(())
}

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct Path {
    pub path: String,
}

pub async fn get_paths(db: &DatabaseConnection) -> Result<Vec<Path>, DbErr> {
    let paths: Vec<Path> = Path::find_by_statement(Statement::from_sql_and_values(
        DbBackend::MySql,
        r#"
            SELECT group_concat(a.ancestor order by a.insertion_order separator "->") as
            path
            FROM closure d
            JOIN closure a
            ON (a.descendant = d.descendant)
            WHERE d.ancestor = "root" and d.ancestor != d.descendant
            GROUP BY a.descendant;
        "#,
        vec![],
    ))
    .all(db)
    .await?;

    return Ok(paths);
}
