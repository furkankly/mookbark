use crate::{
    is_valid_http_url,
    terminal_app::run::{App, Content},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Alignment, Modifier},
    style::{Color, Style, Styled, Stylize},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap},
};
use tui_textarea::TextArea;
use tui_tree_widget::Tree;

pub type Frame<'a> = ratatui::Frame<'a>;

const LIGHT: Color = Color::Rgb(253, 224, 71);
// const LIGHT: Color = Color::LightYellow;
const _MEDIUM: Color = Color::Rgb(141, 95, 28);
const DARK: Color = Color::Rgb(113, 63, 18);
// const DARK: Color = Color::DarkGray;

pub fn validate_input(text_area: &mut TextArea, popup_type: &str) -> bool {
    text_area.set_cursor_style(Style::default());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::new().fg(LIGHT))
        .bg(DARK)
        .padding(Padding::uniform(1));
    let err_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::new().fg(Color::LightRed))
        .bg(DARK)
        .padding(Padding::uniform(1));

    if popup_type == "add-bookmark" {
        text_area.set_placeholder_text("Enter a valid url");
        let first_line = text_area.lines()[0].as_str();
        if first_line.is_empty() || is_valid_http_url(first_line) {
            if first_line.is_empty() {
                text_area.set_block(block.title("Add a bookmark"));
                false
            } else {
                text_area.set_block(block.title("Add a bookmark"));
                true
            }
        } else {
            text_area.set_block(err_block.title("Please provide a valid url"));
            false
        }
    } else {
        text_area.set_placeholder_text("Enter a container name");
        let first_line = text_area.lines()[0].as_str();
        if first_line.is_empty() || !is_valid_http_url(first_line) {
            text_area.set_block(block.title("Add a container"));
            true
        } else {
            text_area.set_block(err_block.title("This seems to be a url, not a container"));
            false
        }
    }
}

pub fn ui(f: &mut Frame<'_>, app: &mut App) {
    tree(f, app);
    popup(f, app);
}

pub fn tree(f: &mut Frame<'_>, app: &mut App) {
    let area = f.size();
    let items = Tree::new(app.tree.items.clone())
        .expect("all item identifiers are unique")
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::new().fg(LIGHT))
                .bold(),
        )
        .highlight_style(Style::new().bg(DARK).fg(LIGHT).add_modifier(Modifier::BOLD))
        .highlight_symbol("~ ");
    f.render_stateful_widget(items, area, &mut app.tree.state);
}

pub fn popup(f: &mut Frame<'_>, app: &mut App) {
    let area = f.size();
    if !app.popup.open.is_empty() {
        let height_percentage = if app.popup.open == "keymaps" { 80 } else { 20 };
        let centered_area = centered_rect(80, height_percentage, area);
        f.render_widget(Clear, centered_area); //this clears out the background

        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(centered_area);
        let content_area = areas[0];
        let actions_area = areas[1];
        match &mut app.popup.content {
            Content::Message(message) => {
                if message.contains("401") && !message.contains("re-authenticate") {
                    message.push_str(". Please quit the app by pressing 'q' and re-authenticate.")
                }
                let content = Paragraph::new(message.clone())
                    .wrap(Wrap { trim: true })
                    .block(
                        Block::default()
                            .title(app.popup.title.clone())
                            .borders(Borders::ALL)
                            .border_type(ratatui::widgets::BorderType::Rounded)
                            .border_style(Style::new().fg(LIGHT))
                            .bg(DARK)
                            .padding(Padding::uniform(1)),
                    );

                f.render_widget(content, content_area);
            }
            Content::TextArea(text_area) => {
                let content = text_area.widget();
                f.render_widget(content, content_area);
            }
        }

        let actions = Paragraph::new(if app.popup.open != "error" {
            "Press 'enter' to submit or 'esc' to close the popup"
        } else {
            "Press 'esc' to close the popup"
        })
        .set_style(Style::default().italic().bold())
        .alignment(Alignment::Center)
        .italic()
        .wrap(Wrap { trim: true })
        .block(Block::default().bg(DARK));
        if app.popup.open != "keymaps" {
            f.render_widget(actions, actions_area);
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
