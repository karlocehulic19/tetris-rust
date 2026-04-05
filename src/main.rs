use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text, ToLine},
    widgets::{Block, Borders, Paragraph, Widget},
};
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let b = Block::default()
        .borders(Borders::ALL)
        .title(Line::from("Tetris").centered());
    let p = render_playing_box(b);
    frame.render_widget(p, frame.area());
}

fn render_playing_box(block: Block) -> Paragraph<'_> {
    let horizontal_border_block = Line::from("🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩".white());
    let vertical_border_block = Line::from("🟩                    🟩".white());
    let mut vector_box: Vec<Line<'_>> = Vec::new();

    let mut horizontal_vector = vec![vertical_border_block; 10];
    vector_box.push(horizontal_border_block.clone());
    vector_box.append(&mut horizontal_vector);
    vector_box.push(horizontal_border_block.clone());

    let playing_box = Text::from(vector_box);
    return Paragraph::new(playing_box).centered().block(block);
}
