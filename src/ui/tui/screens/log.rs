use crate::{
    languages::spoken,
    ui::tui::{self, screens, widgets::ScrollLog, Screen},
    Error, Status,
};
use crossterm::event::{self, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    symbols::border::Set,
    text::{Line, Span},
    widgets::{block::Position, Block, Borders, Clear, Padding, StatefulWidget, Widget},
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::Sender;
use tracing::info;

const TOP_DIALOG_BORDER: Set = Set {
    top_left: "┌",
    top_right: "┐",
    bottom_left: "│",
    bottom_right: "│",
    vertical_left: "│",
    vertical_right: "│",
    horizontal_top: "─",
    horizontal_bottom: " ",
};

const STATUS_BORDER: Set = Set {
    top_left: " ",
    top_right: " ",
    bottom_left: "└",
    bottom_right: "┘",
    vertical_left: " ",
    vertical_right: " ",
    horizontal_top: " ",
    horizontal_bottom: "─",
};

#[derive(Clone, Debug)]
pub struct Log<'a> {
    /// the log messages
    log: VecDeque<(Option<String>, String)>,
    /// max log length
    max_log: usize,
    /// scroll text widget
    st: ScrollLog<'a>,
    /// the cached rect from last render
    area: Rect,
    /// the cached calculated rect
    centered: Rect,
    /// the currently selected spoken language
    spoken_language: Option<spoken::Code>,
}

impl Log<'_> {
    /// Create a new log Screen
    pub fn new(max_log: usize) -> Self {
        let mut st = ScrollLog::default();
        st.scroll_newest();
        Self {
            log: VecDeque::default(),
            max_log,
            st,
            area: Rect::default(),
            centered: Rect::default(),
            spoken_language: None,
        }
    }

    fn recalculate_rect(&mut self, area: Rect) {
        if self.area != area {
            let [_, hc, _] = Layout::horizontal([
                Constraint::Percentage(10),
                Constraint::Min(1),
                Constraint::Percentage(10),
            ])
            .areas(area);
            [_, self.centered, _] = Layout::vertical([
                Constraint::Percentage(10),
                Constraint::Min(1),
                Constraint::Percentage(10),
            ])
            .areas(hc);
            self.area = area;
        }
    }

    fn add_message(&mut self, emoji: Option<String>, msg: String) {
        // add the message to the log
        self.log.push_back((emoji, msg));
        // if the log is too long, remove the oldest message
        if self.log.len() > self.max_log {
            self.log.pop_front();
        }
    }

    // render the log messages
    fn render_log(&mut self, area: Rect, buf: &mut Buffer) {
        // clear
        Widget::render(Clear, area, buf);

        let title = Line::from(vec![
            Span::styled("─", Style::default().fg(Color::DarkGray)),
            Span::styled("/ Log /", Style::default().fg(Color::White)),
        ]);

        let block = Block::default()
            .title(title)
            .title_style(Style::default().fg(Color::White))
            .padding(Padding::horizontal(1))
            .style(Style::default().fg(Color::DarkGray))
            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
            .border_set(TOP_DIALOG_BORDER);

        self.st.block(block);
        self.st.style(Style::default().fg(Color::White));

        // render the scroll text
        StatefulWidget::render(&mut self.st, area, buf, &mut self.log);
    }

    // render the status bar at the bottom
    fn render_status(&mut self, area: Rect, buf: &mut Buffer) {
        let line = Line::from(vec![
            Span::styled("─", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "/ j,k scroll / ⤒ top / ⤓ bottom / ` back / q quit /",
                Style::default().fg(Color::White),
            ),
        ]);
        let block = Block::default()
            .title(line)
            .title_style(Style::default().fg(Color::White))
            .title_position(Position::Bottom)
            .title_alignment(Alignment::Left)
            .style(Style::default().fg(Color::DarkGray))
            .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
            .border_set(STATUS_BORDER)
            .padding(Padding::horizontal(1));

        Widget::render(block, area, buf);
    }

    /// handle UI events
    pub async fn handle_ui_event(
        &mut self,
        event: tui::Event,
        _to_ui: Sender<screens::Event>,
        _status: Arc<Mutex<Status>>,
    ) -> Result<(), Error> {
        match event {
            tui::Event::Log(emoji, msg) => self.add_message(emoji, msg),
            _ => {
                info!("Ignoring UI event: {:?}", event);
            }
        }
        Ok(())
    }

    /// handle input events
    pub async fn handle_input_event(
        &mut self,
        event: event::Event,
        to_ui: Sender<screens::Event>,
        _status: Arc<Mutex<Status>>,
    ) -> Result<(), Error> {
        if let event::Event::Key(key) = event {
            match key.code {
                KeyCode::PageUp => self.st.scroll_oldest(),
                KeyCode::PageDown => self.st.scroll_newest(),
                KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => self.st.scroll_newer(),
                KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => self.st.scroll_older(),
                KeyCode::Char('`') => {
                    info!("input event: Hide Log");
                    to_ui.send((None, tui::Event::ToggleLog).into()).await?
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl Screen for Log<'_> {
    async fn handle_event(
        &mut self,
        event: screens::Event,
        to_ui: Sender<screens::Event>,
        status: Arc<Mutex<Status>>,
    ) -> Result<(), Error> {
        match event {
            screens::Event::Input(input_event) => {
                let spoken = {
                    let status = status.lock().unwrap();
                    status.spoken_language()
                };
                if self.spoken_language != spoken {
                    self.spoken_language = spoken
                }
                self.handle_input_event(input_event, to_ui, status).await
            }
            screens::Event::Ui(_, ui_event) => self.handle_ui_event(ui_event, to_ui, status).await,
        }
    }

    fn render_screen(&mut self, area: Rect, buf: &mut Buffer) -> Result<(), Error> {
        self.recalculate_rect(area);

        // clear area around the popup
        Widget::render(Clear, self.centered, buf);

        let [list_area, status_area] =
            Layout::vertical([Constraint::Percentage(100), Constraint::Min(1)])
                .flex(Flex::End)
                .areas(self.centered);

        self.render_log(list_area, buf);
        self.render_status(status_area, buf);
        Ok(())
    }
}
