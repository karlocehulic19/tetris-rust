use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};
fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Default)]
struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events();
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Line::from("Tetris").centered());

        let horizontal_border = Line::from("🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩".white());
        // one sqare is exectly two spaces
        let vertical_border = Line::from("🟩                    🟩".white());
        let mut vector_box: Vec<Line<'_>> = vec![Line::from(""); 5];

        let mut horizontal_vector = vec![vertical_border; 10];
        vector_box.push(horizontal_border.clone());
        vector_box.append(&mut horizontal_vector);
        vector_box.push(horizontal_border.clone());

        let playing_box = Text::from(vector_box);
        Paragraph::new(playing_box)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
