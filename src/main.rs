use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    game::board,
    general::{colors::Color, dimensions::BOX_HEIGHT, types::ColorBox},
};
mod game;
mod general;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Default)]
struct App {
    exit: bool,
    color_gird: ColorBox,
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

    let mut line_box: Vec<Line<'_>> = Vec::new();
    line_box.push(Line::from("🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩"));
    let mut inner_box = get_inner_box_lines(board.blocks);
    line_box.append(&mut inner_box);
    line_box.push(Line::from("🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩"));

    return line_box;
}

fn get_inner_box_lines<'a>(inner_box: ColorBox) -> Vec<Line<'a>> {
    let mut inner_lines: Vec<Line<'_>> = Vec::new();

    for i in 0..BOX_HEIGHT {
        let mut curr_line: Vec<Span<'_>> = Vec::new();
        curr_line.push(Span::from("🟩"));
        for c in inner_box[i] {
            match c {
                Color::Empty => {
                    curr_line.push(Span::from("  "));
                }
                Color::Green => {
                    curr_line.push(Span::from("🟩").green());
                }
                Color::Red => {
                    curr_line.push(Span::from("🟩").red());
                }
                Color::Yellow => {
                    curr_line.push(Span::from("🟩").yellow());
                }
                Color::Blue => {
                    curr_line.push(Span::from("🟩").blue());
                }
            }
        }
        curr_line.push(Span::from("🟩"));

        inner_lines.push(Line::from(curr_line));
    }

    return inner_lines;
}

fn get_vertical_padding_lines<'a>(padding_size: usize) -> Vec<Line<'a>> {
    let padding = vec![Line::from(""); padding_size];
    return padding;
}
