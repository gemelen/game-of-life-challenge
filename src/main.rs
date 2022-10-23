use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders,
    },
    Frame, Terminal,
};

mod gol;

struct App {
    cells: gol::Colony,
}

impl App {
    // initial state
    fn new(height: i32, width: i32) -> App {
        let glider: Vec<gol::Cell> = vec![
                     (0,-1),
                             (1, 0),
            (-1,1),  (0,1),  (1,1)];

        let centered_glider = glider
            .into_iter()
            .map(|cell| (cell.0 + (width / 2), cell.1 + (height / 2) ))
            .collect();

        App {
            cells: centered_glider,
        }
    }

    fn on_tick(&mut self) {
        self.cells = gol::generation(self.cells.clone());
    }
}

fn run_app<B: Backend>( terminal: &mut Terminal<B>, mut app: App, tick_rate: Duration,) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_millis(500));
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

// sets what's to be drawn in the frame
fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // skip 5% from the top
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints( 
            [
                Constraint::Percentage(5),
                Constraint::Percentage(95)
            ].as_ref()
        )
        .split(f.size());
    // skip 10% from the left
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90)
            ].as_ref()
        )
        .split(vertical_layout[1]);
    // fill canvas with cells
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .paint(|ctx| {
            app.cells.iter().for_each(
                |cell| ctx.draw(
                    &Rectangle {
                        x: cell.0 as f64,
                        y: cell.1 as f64,
                        width: 1.0,
                        height: 1.0,
                        color: Color::White}
                )
            )
        })
        .x_bounds([0.0, 24.0])
        .y_bounds([0.0, 24.0]);
    f.render_widget(canvas, chunks[1]);
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_secs(1);
    let app = App::new(25, 25);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.hide_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
