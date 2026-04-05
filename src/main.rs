use ratatui::{
    DefaultTerminal, Frame,
    prelude::{Color, Span, Style},
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
    let span = Span::styled("🟥🟥🟥🟥", Style::default().fg(Color::Red));

    frame.render_widget(span, frame.area());
}
