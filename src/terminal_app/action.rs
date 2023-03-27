use crate::{
    cli::https_client::SERVER_URL,
    is_valid_http_url,
    terminal_app::{
        run::{App, Content},
        tui::Event,
        ui::validate_input,
    },
};
use crossterm::event::KeyCode;
use reqwest;
use termtree::Tree;
use tui_textarea::{Input, Key};
use tui_tree_widget::TreeItem;

#[derive(Clone, Debug)]
pub enum Action {
    Render,
    Quit,
    TreeToggle,
    TreeLeft,
    TreeRight,
    TreeDown,
    TreeUp,
    TreeFirst,
    TreeLast,
    TreeOpenErrorPopup,
    TreeSetItems,
    TreeOpenAddContainerPopup,
    TreeOpenAddBookmarkPopup,
    TreeOpenDeleteEntityPopup,
    TreeTakePopupAction,
    TreeOpenBookmarkOnBrowser,
    OpenKeymapsPopup,
    ClosePopup,
}

pub fn get_action(app: &mut App, event: Event) -> Option<Action> {
    match event {
        Event::Render => Some(Action::Render),
        Event::Key(key) => {
            if app.popup.open == "add-container" || app.popup.open == "add-bookmark" {
                if let Content::TextArea(text_area) = &mut app.popup.content {
                    let input = Input::from(key);
                    let is_valid = validate_input(text_area, &app.popup.open);
                    match input {
                        Input { key: Key::Esc, .. } => {
                            return Some(Action::ClosePopup);
                        }
                        Input {
                            key: Key::Enter, ..
                        } if is_valid => return Some(Action::TreeTakePopupAction),
                        Input {
                            key: Key::Enter, ..
                        } => {
                            return None;
                        }
                        _ => {
                            if text_area.input(input) {
                                validate_input(text_area, &app.popup.open);
                            }
                            return None;
                        }
                    }
                };
                None
            } else {
                match key.code {
                    KeyCode::Char('q') => Some(Action::Quit),
                    KeyCode::Char('\n' | ' ') if app.popup.open.is_empty() => {
                        Some(Action::TreeToggle)
                    }
                    KeyCode::Left if app.popup.open.is_empty() => Some(Action::TreeLeft),
                    KeyCode::Right if app.popup.open.is_empty() => Some(Action::TreeRight),
                    KeyCode::Down if app.popup.open.is_empty() => Some(Action::TreeDown),
                    KeyCode::Up if app.popup.open.is_empty() => Some(Action::TreeUp),
                    KeyCode::Home if app.popup.open.is_empty() => Some(Action::TreeFirst),
                    KeyCode::End if app.popup.open.is_empty() => Some(Action::TreeLast),
                    KeyCode::Char('r') if app.popup.open.is_empty() => Some(Action::TreeSetItems),
                    KeyCode::Enter => {
                        let selected_entity = selected_entity(app);
                        match selected_entity {
                            Some(selected_entity) => {
                                let selected_entity_type = selected_entity_type(&selected_entity);
                                if app.popup.open.is_empty() {
                                    if selected_entity_type == "bookmark" {
                                        Some(Action::TreeOpenBookmarkOnBrowser)
                                    } else {
                                        None
                                    }
                                } else if app.popup.open != "error" {
                                    Some(Action::TreeTakePopupAction)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    }
                    KeyCode::Esc if !app.popup.open.is_empty() => Some(Action::ClosePopup),
                    KeyCode::Char('?') => Some(Action::OpenKeymapsPopup),
                    KeyCode::Char('c') => {
                        let selected_entity = selected_entity(app);
                        match selected_entity {
                            Some(selected_entity) => {
                                let selected_entity_type = selected_entity_type(&selected_entity);
                                if selected_entity_type == "container" {
                                    Some(Action::TreeOpenAddContainerPopup)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    }
                    KeyCode::Char('b') => {
                        let selected_entity = selected_entity(app);
                        match selected_entity {
                            Some(selected_entity) => {
                                let selected_entity_type = selected_entity_type(&selected_entity);
                                if selected_entity_type == "container" {
                                    Some(Action::TreeOpenAddBookmarkPopup)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    }
                    KeyCode::Char('d') => {
                        let selected_entity = selected_entity(app);
                        match selected_entity {
                            Some(selected_entity) => {
                                if selected_entity != "root" {
                                    Some(Action::TreeOpenDeleteEntityPopup)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    }
                    _ => None,
                }
            }
        }
        _ => None,
    }
}

pub fn update(app: &mut App, action: Action) {
    match action {
        Action::Quit => app.should_quit = true,
        Action::TreeToggle => app.tree.toggle(),
        Action::TreeLeft => app.tree.left(),
        Action::TreeRight => app.tree.right(),
        Action::TreeDown => app.tree.down(),
        Action::TreeUp => app.tree.up(),
        Action::TreeFirst => app.tree.first(),
        Action::TreeLast => app.tree.last(),
        Action::TreeOpenErrorPopup => {
            app.popup.title = String::from("Error");
            app.popup.open = String::from("error");
        }
        Action::TreeSetItems => {
            tokio_scoped::scope(|scope| {
                scope.spawn(async {
                    let result = app
                        .https_client
                        .get(format!("{}/bookmarks", SERVER_URL))
                        .send()
                        .await;
                    match result {
                        Ok(response) => match response.error_for_status() {
                            Ok(response) => {
                                let tree_items = build_tree(
                                    response.json::<termtree::Tree<String>>().await.unwrap(),
                                );
                                app.tree.set_items(vec![tree_items]);
                                app.tree.state.open(vec![String::from("root")]);
                            }
                            Err(err) => {
                                let error_message = err.to_string();
                                app.popup.content = Content::Message(error_message);
                                app.action_tx.send(Action::TreeOpenErrorPopup).unwrap();
                            }
                        },
                        Err(err) => {
                            let error_message = err.to_string();
                            app.popup.content = Content::Message(error_message);
                            app.action_tx.send(Action::TreeOpenErrorPopup).unwrap();
                        }
                    }
                });
            });
        }
        Action::TreeOpenAddContainerPopup => {
            app.popup.title = String::from("Add a container");
            app.popup.open = String::from("add-container");
            let mut text_area = tui_textarea::TextArea::default();
            // To draw the popup
            validate_input(&mut text_area, &app.popup.open);
            app.popup.content = Content::TextArea(text_area);
        }
        Action::TreeOpenAddBookmarkPopup => {
            app.popup.title = String::from("Add a bookmark");
            app.popup.open = String::from("add-bookmark");
            let mut text_area = tui_textarea::TextArea::default();
            // To draw the popup
            validate_input(&mut text_area, &app.popup.open);
            app.popup.content = Content::TextArea(text_area);
        }
        Action::TreeOpenDeleteEntityPopup => {
            // TODO: Can we pass selected_entity with the action so we don't have to unwrap
            // here?
            let selected_entity = selected_entity(app).unwrap();
            if selected_entity_type(&selected_entity) == "bookmark" {
                app.popup.title = String::from("Delete the bookmark");
            } else {
                app.popup.title = String::from("Delete the container");
            };
            app.popup.content = Content::Message( String::from("Want to delete this? In case you already didn't know ~ some actions are irreversible in this life."));
            app.popup.open = String::from("delete");
        }
        Action::TreeTakePopupAction => {
            tokio_scoped::scope(|scope| {
                scope.spawn(async {
                    let selected_entity = selected_entity(app).unwrap();
                    let mut result: Option<Result<reqwest::Response, reqwest::Error>> = None;
                    if app.popup.open == "add-container" {
                        if let Content::TextArea(text_area) = &mut app.popup.content {
                            let query = [
                                ("parent_container_name", &selected_entity),
                                ("container_name", &text_area.lines()[0]),
                            ];
                            result = Some(
                                app.https_client
                                    .post(format!("{}/{}", SERVER_URL, "container"))
                                    .query(&query)
                                    .send()
                                    .await,
                            );
                        }
                    }
                    if app.popup.open == "add-bookmark" {
                        if let Content::TextArea(text_area) = &mut app.popup.content {
                            let query = [
                                ("container_name", &selected_entity),
                                ("bookmark_url", &text_area.lines()[0]),
                            ];
                            result = Some(
                                app.https_client
                                    .post(format!("{}/{}", SERVER_URL, "bookmark"))
                                    .query(&query)
                                    .send()
                                    .await,
                            );
                        }
                    }
                    if app.popup.open == "delete" {
                        let selected_entity_type = selected_entity_type(&selected_entity);
                        let query = [(
                            if selected_entity_type == "bookmark" {
                                "bookmark_url"
                            } else {
                                "container_name"
                            },
                            &selected_entity,
                        )];
                        result = Some(
                            app.https_client
                                .delete(format!("{}/{}", SERVER_URL, selected_entity_type))
                                .query(&query)
                                .send()
                                .await,
                        );
                    }

                    match result.expect("No request has been sent.") {
                        Ok(response) => match response.error_for_status() {
                            Ok(_response) => {
                                if app.action_tx.send(Action::TreeSetItems).is_ok() {
                                    app.action_tx.send(Action::ClosePopup).unwrap()
                                }
                            }
                            Err(err) => {
                                let error_message = err.to_string();
                                app.popup.content = Content::Message(error_message);
                                app.action_tx.send(Action::TreeOpenErrorPopup).unwrap();
                            }
                        },
                        Err(err) => {
                            let error_message = err.to_string();
                            app.popup.content = Content::Message(error_message);
                            app.action_tx.send(Action::TreeOpenErrorPopup).unwrap();
                        }
                    }
                });
            });
        }
        Action::TreeOpenBookmarkOnBrowser => {
            let selected_entity = selected_entity(app).unwrap();
            let _ = webbrowser::open(selected_entity.as_ref());
        }
        Action::OpenKeymapsPopup => {
            app.popup.open = String::from("keymaps");
            app.popup.title = String::from("Keymaps");
            app.popup.content = Content::Message(String::from(
                "
                    Bookmark tree:

                    UpArrow - Move to an upper node
                    DownArrow - Move to a lower node
                    LeftArrow, Space - Close a node
                    RightArrow, Space - Open a node
                    Enter - Open the current node(bookmark) in your default browser
                    c - Add a container to the current node(container)
                    b - Add a bookmark url to the current node(container)
                    d - Delete the current node

                    Popups:

                    Enter - Submit an action in the current popup
                    Esc - Close the current popup

                    q - Quit Mookbark terminal app
                ",
            ));
        }
        Action::ClosePopup => {
            app.popup.open = String::new();
        }
        _ => {}
    }
}

// termtree to tui_tree_widget
fn build_tree<'a>(tree: Tree<String>) -> TreeItem<'a, String> {
    let identifier = tree.root.clone().to_string();
    let text = if identifier.eq("root") {
        String::from("Mookbark")
    } else {
        identifier.clone()
    };
    let mut new_tree = TreeItem::new(identifier, text, vec![]).unwrap();
    for leaf in tree.leaves.into_iter() {
        new_tree.add_child(build_tree(leaf)).unwrap();
    }
    new_tree
}

fn selected_entity(app: &App) -> Option<String> {
    let selected = app.tree.state.selected();
    selected.last().cloned()
}

fn selected_entity_type(selected_entity: &str) -> &str {
    if is_valid_http_url(selected_entity) {
        "bookmark"
    } else {
        "container"
    }
}

// paths to tui_tree_widget
// fn build_tree<'a>(paths: Vec<Vec<&str>>) -> Result<TreeItem<'a, String>> {
//     let mut root = TreeItem::new_leaf(String::from(paths[0][0].clone()), String::from("Root"));
//
//     for path in paths.iter() {
//         let mut current_node = &mut root;
//
//         for &id in path.iter().skip(1) {
//             // Check if the child with the given identifier already exists
//             let child_exists = current_node
//                 .children()
//                 .iter()
//                 .any(|c| c.get_identifier() == &id);
//
//             // If it doesn't exist, add a new child
//             if !child_exists {
//                 let new_child = TreeItem::new_leaf(String::from(id.clone()), format!("{:?}", id));
//                 current_node.add_child(new_child)?;
//             }
//
//             // Find the index of the child with the given identifier
//             let index = current_node
//                 .children()
//                 .iter()
//                 .position(|c| c.get_identifier() == &id)
//                 .unwrap();
//
//             // Move to the next level in the tree
//             current_node = current_node.child_mut(index).unwrap();
//         }
//     }
//
//     Ok(root)
// }
