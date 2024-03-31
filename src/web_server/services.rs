use crate::{
    is_valid_http_url,
    web_server::{
        entities::{prelude::*, *},
        oauth::OAuthUser,
    },
};
use anyhow::{Error, Result};
use sea_orm::TransactionTrait;
use sea_orm::*;

// Closure Table pattern is used for implementing hierarchical data
// See:
// https://stackoverflow.com/questions/17302716/hierarchical-sql-data-recursive-cte-vs-hierarchyid-vs-closure-table

// TODO: user_id is read from sesion and not user table for all services
// can this mess up the data integrity?
// Resetting the session store before each release can help

pub async fn add_bookmark(
    txn: &DatabaseTransaction,
    user_id: &str,
    container_name: &str,
    url: &str,
) -> Result<()> {
    // Validate the bookmark
    if !is_valid_http_url(url) {
        return Err(Error::msg("Not a valid url"));
    }

    // Check if the container exists
    let container = container::Entity::find()
        .filter(container::Column::Name.eq(container_name))
        .one(txn)
        .await?;
    if container.is_none() {
        return Err(Error::msg("Container not found"));
    }

    let bookmark = bookmark::ActiveModel {
        url: ActiveValue::Set(url.to_owned()),
        user_id: ActiveValue::Set(user_id.to_owned()),
        ingested: ActiveValue::Set(false),
    };
    Bookmark::insert(bookmark).exec(txn).await?;

    // Insert for each ancestor of container_name
    txn.execute(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
            INSERT INTO closure (user_id, ancestor, descendant)
            SELECT $1, ancestor, $2
            FROM closure
            WHERE user_id = $1 AND descendant = $3;
        "#,
        vec![user_id.into(), url.into(), container_name.into()],
    ))
    .await?;

    // Ending mark
    txn.execute(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
            INSERT INTO closure (user_id, ancestor, descendant)
            SELECT $1, $2, $2;
        "#,
        vec![user_id.into(), url.into()],
    ))
    .await?;
    Ok(())
}

pub async fn add_container(
    txn: &DatabaseTransaction,
    user_id: &str,
    parent_container_name: &str,
    container_name: &str,
) -> Result<()> {
    // Validate the container
    // TODO: Doing this because apps render based on this validation, this could be improved by
    // explicit entity types
    if is_valid_http_url(container_name) {
        return Err(Error::msg("Not a valid container"));
    }

    // Check if the parent container exists
    if container_name != "root" {
        let parent_container = container::Entity::find()
            .filter(container::Column::Name.eq(parent_container_name))
            .one(txn)
            .await?;
        if parent_container.is_none() {
            return Err(Error::msg("Parent container not found"));
        }
    }

    let container = container::ActiveModel {
        name: ActiveValue::Set(container_name.to_owned()),
        user_id: ActiveValue::Set(user_id.to_owned()),
    };
    Container::insert(container).exec(txn).await?;

    if container_name == "root" {
        txn.execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                INSERT INTO closure (user_id, ancestor, descendant)
                SELECT $1, $2, $3;
            "#,
            // parent_container_name = "root" , container_name = "root"
            vec![
                user_id.into(),
                parent_container_name.into(),
                container_name.into(),
            ],
        ))
        .await?;
    } else {
        // Insert for each ancestor of parent_container_name
        txn.execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                INSERT INTO closure (user_id, ancestor, descendant)
                SELECT $1, ancestor, $2
                FROM closure
                WHERE user_id = $1 AND descendant = $3;
            "#,
            vec![
                user_id.into(),
                container_name.into(),
                parent_container_name.into(),
            ],
        ))
        .await?;

        // Ending mark
        txn.execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                INSERT INTO closure (user_id, ancestor, descendant)
                SELECT $1, $2, $2;
            "#,
            vec![user_id.into(), container_name.into()],
        ))
        .await?;
    }

    Ok(())
}

pub async fn delete_entity(
    db_conn: &DatabaseConnection,
    user_id: &str,
    entity_type: &str,
    url_or_container_name: &str,
) -> Result<()> {
    let txn = db_conn.begin().await?;

    // Check if trying to delete "root", if so prevent with error
    if url_or_container_name == "root" {
        return Err(Error::msg("Trying to delete root"));
    }

    // Check if trying to delete an entity which doesn't exist
    if entity_type == "container" {
        let container = container::Entity::find()
            .filter(container::Column::Name.eq(url_or_container_name))
            .one(&txn)
            .await?;
        if container.is_none() {
            return Err(Error::msg("Container not found"));
        }
    } else {
        let bookmark = bookmark::Entity::find()
            .filter(bookmark::Column::Url.eq(url_or_container_name))
            .one(&txn)
            .await?;
        if bookmark.is_none() {
            return Err(Error::msg("Bookmark not found"));
        }
    }

    // Try deleting from bookmark in case its a bookmark or a container that has a bookmark in the
    // path of the container (as a leaf node)
    //
    // PlanetScale doesn't support CTE yet, and temporary tables
    // raise db errors too for some reason. So we subquery the paths in each step of deletion.
    txn.execute(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
            DELETE FROM bookmark
            WHERE user_id = $1
                AND url IN(
                    SELECT descendant
                    FROM closure
                    WHERE ancestor = $2);
        "#,
        vec![user_id.into(), url_or_container_name.into()],
    ))
    .await?;

    if entity_type.eq("container") {
        // Try deleting from container in case its a container
        txn.execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
                DELETE FROM container 
                WHERE user_id = $1
                    AND name IN(
                        SELECT descendant
                        FROM closure
                        WHERE ancestor = $2);
            "#,
            vec![user_id.into(), url_or_container_name.into()],
        ))
        .await?;
    }

    // Delete the path(s)
    // TODO: Research if this is the case with Postgres as well:
    // MySQL doesn't like to directly access a table it is deleting from so we need to wrap the
    // table in a pseudo subquery.
    // https://stackoverflow.com/questions/4471277/mysql-delete-from-with-subquery-as-condition
    txn.execute(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
            DELETE FROM closure
            WHERE user_id = $1
                AND descendant IN(
                    SELECT descendant 
                    FROM(
                        SELECT descendant 
                        FROM closure
                        WHERE ancestor = $2)
                    AS d);
        "#,
        vec![user_id.into(), url_or_container_name.into()],
    ))
    .await?;

    txn.commit().await?;
    Ok(())
}
#[derive(Debug, FromQueryResult)]
pub struct Path {
    pub path: String,
}

pub async fn get_paths(db_conn: &DatabaseConnection, user_id: &str) -> Result<Vec<Path>, DbErr> {
    let paths: Vec<Path> = Path::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
            SELECT STRING_AGG(a.ancestor, '->' ORDER BY a.closure_id) AS path
            FROM closure AS d
            JOIN closure AS a ON a.descendant = d.descendant
            WHERE d.user_id = $1
                AND a.user_id = $1
                AND d.ancestor = 'root'
                AND d.ancestor != d.descendant
            GROUP BY a.descendant;
        "#,
        vec![user_id.into()],
    ))
    .all(db_conn)
    .await?;
    Ok(paths)
}

pub async fn get_bookmark(
    db_conn: &DatabaseConnection,
    user_id: &str,
    url: &str,
) -> Result<Option<bookmark::Model>, DbErr> {
    let bookmark = bookmark::Entity::find()
        .filter(bookmark::Column::UserId.eq(user_id.as_str()))
        .filter(bookmark::Column::Url.eq(url))
        .one(db_conn)
        .await?;

    Ok(bookmark)
}

pub async fn add_user(
    db_conn: &DatabaseConnection,
    oauth_provider: &str,
    user_id: &str,
    oauth_user: OAuthUser,
) -> Result<()> {
    let txn = db_conn.begin().await?;

    let user = user::ActiveModel {
        user_id: ActiveValue::Set(user_id.to_string()),
        email: ActiveValue::Set(oauth_user.email.clone()),
        username: ActiveValue::Set(oauth_user.username.clone()),
        avatar: ActiveValue::Set(oauth_user.avatar.clone()),
        oauth_provider: ActiveValue::Set(oauth_provider.to_string()),
        oauth_user_id: ActiveValue::Set(oauth_user.id.clone()),
    };
    user::Entity::insert(user).exec(&txn).await?;

    // Create a root node for the new user
    add_container(&txn, user_id, "root", "root").await?;

    txn.commit().await?;
    Ok(())
}

pub async fn check_user(
    db_conn: &DatabaseConnection,
    oauth_provider: &str,
    oauth_user_id: &str,
) -> Result<Option<user::Model>, DbErr> {
    let user = user::Entity::find()
        .filter(user::Column::OauthProvider.eq(oauth_provider))
        .filter(user::Column::OauthUserId.eq(oauth_user_id))
        .one(db_conn)
        .await?;
    Ok(user)
}

pub async fn get_user(
    db_conn: &DatabaseConnection,
    user_id: &str,
) -> Result<Option<user::Model>, DbErr> {
    let user = user::Entity::find()
        .filter(user::Column::UserId.eq(user_id))
        .one(db_conn)
        .await?;
    Ok(user)
}
