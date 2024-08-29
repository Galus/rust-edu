use crate::tui;
use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::{border, Marker},
    text::{Line as TextLine, Text},
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Rectangle},
        Block, Paragraph, Widget,
    },
    Frame,
};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
#[derive(Debug)]
pub struct App {
    counter: u8,
    exit: bool,
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Default for App {
    fn default() -> Self {
        let screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        Self {
            counter: 0,
            exit: false,
            screen,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter()?,
            KeyCode::Right => self.increment_counter()?,
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    // galus: There is an overflow bug here left for educational porpoises 🎓 🐬
    fn increment_counter(&mut self) -> Result<()> {
        self.counter += 1;
        if self.counter > 2 {
            bail!("counter overflow");
        }
        Ok(())
    }

    fn decrement_counter(&mut self) -> Result<()> {
        self.counter -= 1;
        Ok(())
    }

    fn content(&self) -> impl Widget + '_ {
        let mut screen = self.screen.clone();
        screen[1000..1099].copy_from_slice(&[true; 99]);

        let canvas = Canvas::default()
            .marker(Marker::Block)
            .block(Block::bordered().title("Canvas"))
            .x_bounds([0.0, SCREEN_WIDTH as f64])
            .y_bounds([0.0, SCREEN_HEIGHT as f64])
            .paint(move |ctx| {
                for y in 0..SCREEN_HEIGHT {
                    for x in 0..SCREEN_WIDTH {
                        let index = y * SCREEN_WIDTH + x;
                        if screen[index] {
                            ctx.draw(&Rectangle {
                                x: x as f64,
                                y: y as f64,
                                width: 1.0,
                                height: 1.0,
                                color: Color::Cyan,
                            })
                        }
                    }
                }
            });
        canvas
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(TextLine::from(vec![
            " Canvas ".bold(),
            "<3".red().bold(),
            " Galus ".bold(),
        ]));

        let instructions = Title::from(TextLine::from(vec![
            " Left ".into(),
            "<H> ".blue().bold(),
            " Right ".into(),
            "<L> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Right))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![TextLine::from(vec![
            " Value: ".into(),
            self.counter.to_string().yellow(),
            " ".into(),
        ])]);

        let paragraph = Paragraph::new(counter_text).alignment(Alignment::Center);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(block.inner(area));

        block.render(area, buf);
        paragraph.render(chunks[0], buf);
        self.content().render(chunks[1], buf);
    }
}
