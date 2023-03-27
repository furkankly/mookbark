use crate::{
    cli::https_client::get_https_client,
    terminal_app::{
        action::{get_action, update, Action},
        stateful_tree::StatefulTree,
        tui::Tui,
        ui::ui,
    },
};
use anyhow::Result;
use tokio::sync::mpsc::UnboundedSender;

pub enum Content<'a> {
    Message(String),
    TextArea(tui_textarea::TextArea<'a>),
}

pub struct Popup<'a> {
    // open can be "add-container" | "add-bookmark" | "delete" | "error" | "keymaps"
    pub open: String,
    pub title: String,
    pub content: Content<'a>,
}
// App state
pub struct App<'a> {
    pub action_tx: UnboundedSender<Action>,
    pub https_client: reqwest::Client,
    pub should_quit: bool,
    pub popup: Popup<'a>,
    pub tree: StatefulTree<'a>,
}

pub async fn run() -> Result<()> {
    // ratatui terminal
    let mut tui = Tui::new()?.frame_rate(30.0);
    tui.enter()?;

    let (action_tx, mut action_rx) = tokio::sync::mpsc::unbounded_channel();

    // application state
    let mut app = App {
        action_tx: action_tx.clone(),
        https_client: get_https_client().await?,
        should_quit: false,
        popup: Popup {
            open: String::new(),
            title: String::new(),
            content: Content::Message(String::from("")),
        },
        tree: StatefulTree::with_items(vec![]),
    };

    action_tx.send(Action::TreeSetItems)?;

    loop {
        let event = tui.next().await?; // blocks until next event
        let action = get_action(&mut app, event);
        if let Some(action) = action {
            action_tx.send(action)?;
        }

        while let Ok(action) = action_rx.try_recv() {
            // application update
            update(&mut app, action.clone());
            // render only when we receive Action::Render
            if let Action::Render = action {
                tui.draw(|f| {
                    ui(f, &mut app);
                })?;
            }
        }

        // application exit
        if app.should_quit {
            break;
        }
    }
    tui.exit()?;

    Ok(())
}
