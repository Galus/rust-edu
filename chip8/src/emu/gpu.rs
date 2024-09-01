// Contains the graphics processing.
use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

use ratatui::{
    backend::CrosstermBackend,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::{border, Marker},
    text::Line as TextLine,
    text::Text,
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Rectangle},
        Block, Paragraph, Widget,
    },
    Frame, Terminal,
};

use std::io::{self, stdout, Stdout};
/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
#[derive(Debug)]
pub struct Gpu {
    counter: u8,
    exit: bool,
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Default for Gpu {
    fn default() -> Self {
        let screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        Self {
            counter: 0,
            exit: false,
            screen,
        }
    }
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            counter: 0,
            exit: false,
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            //

            // Render
            terminal.draw(|frame| self.render_frame(frame))?;

            // Handle Input
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

    /// Initialize the terminal
    pub fn init(&self) -> io::Result<Tui> {
        execute!(stdout(), EnterAlternateScreen)?;
        // What is raw_mode?
        //   Starts taking input immediately w/o waiting for newline
        //   and prevents typed keys being echo'd back
        enable_raw_mode()?;
        Self::set_panic_hook();
        Terminal::new(CrosstermBackend::new(stdout()))
    }

    /// Restore the terminal to its original state
    pub fn restore() -> io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn set_panic_hook() {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Self::restore();
            hook(panic_info);
        }))
    }
}

impl Widget for &Gpu {
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

//use ratatui::{
//    backend::CrosstermBackend,
//    crossterm::{
//        execute,
//        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//    },
//    Terminal,
//};

//
//use ratatui::{
//    style::Color,
//    widgets::{canvas::*, *},
//};
//
//Canvas::default()
//    .block(Block::bordered().title("Canvas"))
//    .x_bounds([-180.0,180.0])
//    .y_bounds([-90.0,90.0])
//    .paint(|ctx| {
//        ctx.draw(&Map {
//            resolution: MapResolution::High,
//            color: Color::White,
//        });
//        ctx.layer();
//        ctx.draw(&Line {
//            x1: 0.0,
//            y1: 10.0,
//            x2: 10.0,
//            y2: 10.0,
//            color: Color::White,
//        });
//        ctx.draw(&Rectangle {
//            x: 10.0,
//            y: 20.0,
//            width: 10.0,
//            height: 10.0,
//            color: Color::Red,
//        });
//    });
