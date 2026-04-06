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

use crate::game::board;
mod game;

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

        let mut game_vec: Vec<Line<'_>> = Vec::new();
        game_vec.append(&mut get_vertical_padding_lines(1));
        game_vec.append(&mut get_next_block());
        game_vec.append(&mut get_vertical_padding_lines(3));
        game_vec.append(&mut get_border_lines());

        let game_ui = Text::from(game_vec);
        Paragraph::new(game_ui)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

fn get_next_block<'a>() -> Vec<Line<'a>> {
    let placing_block_padding = Line::from("");
    let upper = Line::from("      🟩").red();
    let lower = Line::from("🟩🟩🟩🟩").red();

    let placing = vec![placing_block_padding, upper, lower];
    return placing;
}

fn get_border_lines<'a>() -> Vec<Line<'a>> {
    let mut board = board::Board::new();
    board.place_block();

    let box_strings = get_playing_box_strings(board.to_vec_strings());
    let mut line_box: Vec<Line<'_>> = Vec::new();
    for s in box_strings {
        line_box.push(Line::from(s));
    }

    return line_box;
}

fn get_playing_box_strings(inner_box: [String; 10]) -> Vec<String> {
    let top_btm_border = "🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩".to_string();
    let mut box_strings: Vec<String> = Vec::new();
    box_strings.push(top_btm_border.clone());
    for i in 0..10 {
        let mut row = String::new();
        row.push('🟩');
        let inner_row = inner_box[i].clone().replace("0", "  ").replace("1", "🟩");
        row += &inner_row;
        row.push('🟩');
        box_strings.push(row);
    }

    box_strings.push(top_btm_border.clone());
    return box_strings;
}

fn get_vertical_padding_lines<'a>(padding_size: usize) -> Vec<Line<'a>> {
    let padding = vec![Line::from(""); padding_size];
    return padding;
}
