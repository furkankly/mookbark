use crate::services::{get_paths, Path};

use sea_orm::{DatabaseConnection, DbErr};
use termtree::Tree;

fn parse_paths(paths: &mut Vec<Path>) -> Vec<Vec<&str>> {
    let mut result = vec![];
    for path in paths {
        let parsed_path: Vec<&str> = path.path.split("->").collect();
        result.push(parsed_path);
    }
    result
}

fn find_child_node<'a>(
    leaves: &'a mut Vec<Tree<String>>,
    node: &str,
) -> Option<&'a mut Tree<String>> {
    for leaf in leaves.iter_mut() {
        if leaf.root == node {
            return Some(leaf);
        }
    }
    return None;
}

fn traverse(tree: &mut Tree<String>, mut path: Vec<&str>) {
    path.drain(0..1);
    if path.len() > 0 {
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

pub async fn get_bookmarks(db: &DatabaseConnection) -> Result<Tree<String>, DbErr> {
    let mut paths = get_paths(&db).await?;
    let parsed_paths = parse_paths(&mut paths);
    let mut tree = Tree::new(String::from("root"));
    for path in parsed_paths {
        traverse(&mut tree, path);
    }
    return Ok(tree);
}
