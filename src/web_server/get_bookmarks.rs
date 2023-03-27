use crate::web_server::{create_server::AppState, services, services::Path};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, DbErr};
use serde_json::{json, Value};
use std::sync::Arc;
use termtree::Tree;
use tower_sessions::Session;

pub async fn get_bookmarks(
    session: Session,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let Some(user_id) = session.get::<String>("user_id").expect("Couldn't deserialize session")
    else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let result = build_bookmarks_tree(&app_state.db_conn, user_id.as_str()).await;
    match result {
        Ok(bookmarks) => Ok(Json(json!(bookmarks))),
        Err(_err) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn build_bookmarks_tree(
    db: &DatabaseConnection,
    user_id: &str,
) -> Result<Tree<String>, DbErr> {
    let mut paths = services::get_paths(db, user_id).await?;
    let parsed_paths = parse_paths(&mut paths);
    let mut tree = Tree::new(String::from("root"));
    for path in parsed_paths {
        traverse(&mut tree, path);
    }
    Ok(tree)
}

pub fn parse_paths(paths: &mut Vec<Path>) -> Vec<Vec<&str>> {
    let mut result = vec![];
    for path in paths {
        let parsed_path: Vec<&str> = path.path.split("->").collect();
        result.push(parsed_path);
    }
    result
}

fn traverse(tree: &mut Tree<String>, mut path: Vec<&str>) {
    path.drain(0..1);
    if !path.is_empty() {
        let child_node = find_child_node(&mut tree.leaves, path[0]);
        match child_node {
            Some(child_node) => {
                traverse(child_node, path);
            }
            None => {
                let new_child_node = Tree::new(String::from(path[0]));
                tree.push(new_child_node);
                let child_node = find_child_node(&mut tree.leaves, path[0])
                    .expect("Should find the child node that is just pushed into the tree");
                traverse(child_node, path);
            }
        }
    }
}

fn find_child_node<'a>(leaves: &'a mut [Tree<String>], node: &str) -> Option<&'a mut Tree<String>> {
    leaves.iter_mut().find(|leaf| leaf.root.eq(node))
}

// Using slices? (Requires semantic refactor)
// fn traverse(tree: &mut Tree<String>, path: &[&str]) {
//     if let [current, rest @ ..] = path {
//         let child_node = find_child_node(&mut tree.leaves, current);
//         match child_node {
//             Some(child_node) => {
//                 traverse(child_node, rest);
//             }
//             None => {
//                 let mut new_child_node = Tree::new(String::from(current));
//                 traverse(&mut child_node, path);
//                 tree.push(new_child_node);
//             }
//         }
//     }
// }
