use ratatui::{crossterm::event, style::Color, widgets::Widget};
use ratatui_stacked_bar::StackedSparkline;

fn main() {
    let mut terminal = ratatui::init();

    loop {
        let widget = StackedSparkline::default()
            .add_data((30..110).map(|x| x / 4).collect(), Color::Red)
            .add_data((40..120).map(|x| x / 4).collect(), Color::Green)
            .add_data((20..100).map(|x| x / 4).collect(), Color::Blue)
            .max(120);
        terminal
            .draw(|frame| widget.render(frame.area(), frame.buffer_mut()))
            .unwrap();

        match event::read() {
            Ok(event::Event::Key(key_event)) if key_event.kind == event::KeyEventKind::Press => {
                break;
            }
            _ => {}
        }
    }

    ratatui::restore();
}
