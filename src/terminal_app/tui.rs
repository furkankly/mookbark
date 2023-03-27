use anyhow::Result;
use crossterm::{
    cursor,
    event::{Event as CrosstermEvent, KeyEvent, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::{backend::CrosstermBackend as Backend, Terminal};
use std::ops::{Deref, DerefMut};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub enum Event {
    Init,
    Error,
    Render,
    FocusGained,
    FocusLost,
    // Mouse(MouseEvent),
    // Paste(String),
    Key(KeyEvent),
    Resize(u16, u16),
}

pub struct Tui {
    pub terminal: Terminal<Backend<std::io::Stderr>>,
    pub task: JoinHandle<()>,
    pub cancellation_token: CancellationToken,
    pub event_tx: UnboundedSender<Event>,
    pub event_rx: UnboundedReceiver<Event>,
    // pub mouse: bool,
    // pub paste: bool,
    pub frame_rate: f64,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = ratatui::Terminal::new(Backend::new(std::io::stderr()))?;
        let task = tokio::spawn(async {});
        let cancellation_token = CancellationToken::new();
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        // let mouse = false;
        // let paste = false;
        let frame_rate = 60.0;
        Ok(Self {
            terminal,
            task,
            cancellation_token,
            event_tx,
            event_rx,
            // mouse,
            // paste,
            frame_rate,
        })
    }
    pub fn frame_rate(mut self, frame_rate: f64) -> Self {
        self.frame_rate = frame_rate;
        self
    }

    // pub fn mouse(mut self, mouse: bool) -> Self {
    //     self.mouse = mouse;
    //     self
    // }
    //
    // pub fn paste(mut self, paste: bool) -> Self {
    //     self.paste = paste;
    //     self
    // }

    pub fn start(&mut self) {
        let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
        self.cancel();
        self.cancellation_token = CancellationToken::new();
        let _cancellation_token = self.cancellation_token.clone();
        let _event_tx = self.event_tx.clone();
        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut render_interval = tokio::time::interval(render_delay);
            _event_tx.send(Event::Init).unwrap();
            loop {
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = _cancellation_token.cancelled() => {
                        break;
                    }
                    maybe_event = crossterm_event => {
                    match maybe_event {
                        Some(Ok(evt)) => {
                        match evt {
                                CrosstermEvent::Key(key) => {
                                    if key.kind == KeyEventKind::Press {
                                        _event_tx.send(Event::Key(key)).unwrap();
                                    }
                                },
                                CrosstermEvent::Resize(x, y) => {
                                    _event_tx.send(Event::Resize(x, y)).unwrap();
                                },
                                CrosstermEvent::FocusLost => {
                                    _event_tx.send(Event::FocusLost).unwrap();
                                },
                                CrosstermEvent::FocusGained => {
                                    _event_tx.send(Event::FocusGained).unwrap();
                                },
                                // CrosstermEvent::Mouse(mouse) => {
                                //     _event_tx.send(Event::Mouse(mouse)).unwrap();
                                // },
                                // CrosstermEvent::Paste(s) => {
                                //     _event_tx.send(Event::Paste(s)).unwrap();
                                // },
                                _ => {}
                            }
                        }
                        Some(Err(_)) => {_event_tx.send(Event::Error).unwrap(); }
                        None => {},
                        }
                    }
                    _ = render_delay => {
                        _event_tx.send(Event::Render).unwrap();
                    }
                }
            }
        })
    }
    pub fn cancel(&mut self) {
        self.cancellation_token.cancel();
    }
    pub fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), EnterAlternateScreen, cursor::Hide)?;
        // if self.paste {
        //     crossterm::execute!(std::io::stderr(), EnableBracketedPaste)?;
        // }
        // if self.mouse {
        //     crossterm::execute!(std::io::stderr(), EnableMouseCapture)?;
        // }
        self.start();
        Ok(())
    }
    pub fn exit(&mut self) -> Result<()> {
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.flush()?;
            // if self.paste {
            //     crossterm::execute!(std::io::stderr(), DisableBracketedPaste)?;
            // }
            // if self.mouse {
            //     crossterm::execute!(std::io::stderr(), DisableMouseCapture)?;
            // }
            crossterm::execute!(std::io::stderr(), LeaveAlternateScreen, cursor::Show)?;
            crossterm::terminal::disable_raw_mode()?;
        }
        Ok(())
    }
    pub async fn next(&mut self) -> Result<Event> {
        self.event_rx
            .recv()
            .await
            .ok_or(anyhow::anyhow!("Unable to receive event!"))
    }
}

impl Deref for Tui {
    type Target = ratatui::Terminal<Backend<std::io::Stderr>>;
    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
