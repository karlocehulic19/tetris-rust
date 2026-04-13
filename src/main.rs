use std::{
    fmt::Debug,
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

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
    game::board::Board,
    general::{
        colors::Color,
        dimensions::{BOX_HEIGHT, BOX_WIDTH},
        movements::Movement,
        types::ColorBox,
    },
};
mod game;
mod general;

fn main() -> io::Result<()> {
    let (c_tx, c_rx) = mpsc::channel();
    let (e_tx, e_rx) = mpsc::channel();

    let mut board = Board::new(e_tx, c_rx);
    let app = App::new(c_tx, e_rx);
    thread::spawn(move || {
        board.start_game();
    });

    ratatui::run(|terminal| app.run_fn(terminal))
}

#[derive(Debug)]
struct App {
    exit: bool,
    color_gird: ColorBox,
    command_sender: Sender<Movement>,
    evenet_receiver: Receiver<ColorBox>,
}

impl App {
    fn new(c_tx: Sender<Movement>, e_rx: Receiver<ColorBox>) -> Self {
        Self {
            exit: false,
            color_gird: [[Color::Empty; BOX_WIDTH]; BOX_HEIGHT],
            command_sender: c_tx,
            evenet_receiver: e_rx,
        }
    }

    fn run_fn(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            if self.exit {
                return Ok(());
            }

            let has_event = event::poll(Duration::from_secs(0))?;
            if has_event {
                let event = event::read()?;
                self.handle_event(event);
            }

            let received = self.evenet_receiver.try_recv();
            match received {
                Ok(new_board) => {
                    self.update_color_box(new_board);
                }
                Err(_) => {}
            }

            terminal.draw(|frame| self.draw(frame));
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('l') => {
                self.command_sender.send(Movement::Right).unwrap();
            }
            KeyCode::Char('h') => {
                self.command_sender.send(Movement::Left).unwrap();
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn update_color_box(&mut self, new_board: ColorBox) {
        self.color_gird = new_board.clone()
    }

    fn get_border_lines<'a>(&self) -> Vec<Line<'a>> {
        let mut line_box: Vec<Line<'_>> = Vec::new();
        line_box.push(Line::from("🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩"));
        let mut inner_box = get_inner_box_lines(self.color_gird);
        line_box.append(&mut inner_box);
        line_box.push(Line::from("🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩"));

        return line_box;
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
        game_vec.append(&mut self.get_border_lines());

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
