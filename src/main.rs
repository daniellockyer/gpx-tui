#[macro_use]
extern crate clap;
extern crate gpx;

use std::io::{self, BufReader};
use std::fs::File;
use std::cmp::Ordering::Equal;

use clap::{App, Arg};

use gpx::read;
use gpx::{Gpx, Track, TrackSegment};

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::style::Color;
use tui::widgets::{Widget, Block, Borders};
use tui::widgets::canvas::{Canvas, Map, MapResolution};
use tui::layout::{Layout, Constraint, Direction};
use termion::raw::IntoRawMode;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new(crate_name!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap_or("example.gpx");
    let f = File::open(input_file)?;
    let reader = BufReader::new(f);
    let gpx: Gpx = read(reader).unwrap();

    let track: &Track = &gpx.tracks[0];
    let segment: &TrackSegment = &track.segments[0];
    let points = &segment.points;

    let collected: Vec<(f64, f64)> = points.iter().map(|p| (p.point().x(), p.point().y())).collect();

    let min_x = collected.iter().min_by(|p1, p2| p1.0.partial_cmp(&p2.0).unwrap_or(Equal)).unwrap().0;
    let max_x = collected.iter().max_by(|p1, p2| p1.0.partial_cmp(&p2.0).unwrap_or(Equal)).unwrap().0;
    let min_y = collected.iter().min_by(|p1, p2| p1.1.partial_cmp(&p2.1).unwrap_or(Equal)).unwrap().1;
    let max_y = collected.iter().max_by(|p1, p2| p1.1.partial_cmp(&p2.1).unwrap_or(Equal)).unwrap().1;

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
            .split(f.size());

        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("World"))
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::White,
                    resolution: MapResolution::High,
                });

                for p in &collected {
                    ctx.print(p.0, p.1, "X", Color::Green);
                }
            })
            .x_bounds([min_x, max_x])
            .y_bounds([min_y, max_y])
            .render(&mut f, chunks[0]);
    })
}
