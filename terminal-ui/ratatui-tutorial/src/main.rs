use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    // init terminal
    //  We define 'tui' module later.
    let mut terminal = tui::init()?;
    // run app
    //  We define 'App' type later   (undeclared type App: we define App later)
    let app_result = App::default().run(&mut terminal);
    // restore original terminal
    if let Err(err) = tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    app_result
}

mod tui;

// Default is cool, allows App.default() to generate counter=0 and exit=false
#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            // galus: clojure frame must render the entire UI.
            //  only call Terminal::draw() once per loop
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    // galus: I am not used to writing code top-down
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        // event::read() blocks
        //  if the app needs to do more than just UI (think Chip8)
        //  then we can check for pending events with event::poll + timeout (covered in future ch.)
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
}

impl Widget for &App {
    // render Summary
    // - creates a block with a title
    // - instr txt footer
    // - borders
    // - paragraph Widget w/ counter's state
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Ratatui Tutorial <3 from Galus ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK); // galus: All caps made me smile THICC

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 80, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━ Counter App Ratatui Tutorial <3 from Galus ━━━━━━━━━━━━━━━━━┓",
            "┃                                   Value: 0                                   ┃",
            "┃                                                                              ┃",
            "┗━━━━━━━━━━━━━━━━ Decrement <Left> Increment <Right> Quit <Q> ━━━━━━━━━━━━━━━━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();

        /* galus: ORIGINAL
        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);
        */

        /* galus: failing test output
                *assert_buffer_eq
        styles: [
                x: 0, y: 0, fg: Reset, bg: Reset, underline: Reset, modifier: NONE,
                x: 18, y: 0, fg: Reset, bg: Reset, underline: Reset, modifier: BOLD,
                x: 62, y: 0, fg: Reset, bg: Reset, underline: Reset, modifier: NONE,
                x: 43, y: 1, fg: Yellow, bg: Reset, underline: Reset, modifier: NONE,
                x: 44, y: 1, fg: Reset, bg: Reset, underline: Reset, modifier: NONE,
                x: 28, y: 3, fg: Blue, bg: Reset, underline: Reset, modifier: BOLD,
                x: 34, y: 3, fg: Reset, bg: Reset, underline: Reset, modifier: NONE,
                x: 45, y: 3, fg: Blue, bg: Reset, underline: Reset, modifier: BOLD,
                x: 52, y: 3, fg: Reset, bg: Reset, underline: Reset, modifier: NONE,
                x: 58, y: 3, fg: Blue, bg: Reset, underline: Reset, modifier: BOLD,
                x: 62, y: 3, fg: Reset, bg: Reset, underline: Reset, modifier: NONE,
            ]
        */

        /* galus: Analysis
         *  Rect::new(x,y,width,height)
         *  mine:
         *  len(" Counter App Ratatui Tutorial <3 from Galus ") == 44
         *  original:
         *  len(" Counter App Tutorial ") == 22
         *  original starts at 14x and mine 18x
         */
        expected.set_style(Rect::new(18, 0, 44, 1), title_style);
        expected.set_style(Rect::new(43, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(28, 3, 6, 1), key_style);
        expected.set_style(Rect::new(45, 3, 7, 1), key_style);
        expected.set_style(Rect::new(58, 3, 4, 1), key_style);

        // note ratatui also has an assert_buffer_eq! macro that can be used to
        // compare buffers and display the differences in a more readable way
        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into()).unwrap();
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into()).unwrap();
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert!(app.exit);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn handle_key_event_panic() {
        let mut app = App::default();
        let _ = app.handle_key_event(KeyCode::Left.into());
    }

    #[test]
    fn handle_key_event_overflow() {
        let mut app = App::default();
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert_eq!(
            app.handle_key_event(KeyCode::Right.into())
                .unwrap_err()
                .to_string(),
            "counter overflow"
        );
    }
}
