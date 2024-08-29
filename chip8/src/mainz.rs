use color_eyre::Result;
mod app;
mod tui;

fn main() -> Result<()> {
    color_eyre::install()?; // error hooks
    let mut terminal = tui::init()?;

    let app_result = app::App::default().run(&mut terminal);

    if let Err(err) = tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    app_result
}
