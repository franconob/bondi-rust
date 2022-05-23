use cursive::direction::{Absolute, Direction, Relative};
use cursive::event::EventResult;
use cursive::traits::{Nameable, Scrollable};
use cursive::views::{LinearLayout, Panel, SelectView};
use cursive::{Cursive, View};
use log::log;

use bondi::bus_lines::{self, Line};
use bondi::bus_lines::{ListDisplay, Street};
use flexi_logger::{Duplicate, FileSpec, Logger};
use std::collections::HashMap;
use std::error::Error;

struct AppState {
    bus_lines: Vec<Line>,
    bus_streets: Vec<Street>,
    bus_intersec: Vec<Street>,
    selected_line: Option<Line>,
    selected_street: Option<Line>,
}

impl AppState {
    fn new(items: Vec<Line>) -> Self {
        AppState {
            bus_lines: items,
            bus_streets: Vec::new(),
            bus_intersec: Vec::new(),
            selected_line: None,
            selected_street: None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().suppress_timestamp()) // write logs to file
        .duplicate_to_stderr(Duplicate::Warn) // print warnings and errors also to the console
        .start()?;

    let lines = bus_lines::get_lines()?;
    let app = AppState::new(lines);
    let mut siv = cursive::default();
    ui(&mut siv, &app);

    siv.add_global_callback('q', |s| s.quit());

    siv.run();

    Ok(())
}

fn ui(siv: &mut Cursive, app: &AppState) {
    let mut select_line = SelectView::<String>::new().with_all(
        app.bus_lines
            .iter()
            .map(|line| (line.get_text().to_string(), line.line_id.to_string())),
    );

    select_line.set_on_submit(|s, line_id: &String| {
        handle_line_select(s, line_id);
    });

    let select_street = SelectView::<String>::new().with_name("select_street");
    let select_intersection = SelectView::<String>::new().with_name("select_intersection");

    let linear_layout = LinearLayout::horizontal();
    let panel = Panel::new(select_line).title("Lineas").scrollable();
    let panel_streets = Panel::new(select_street).title("Calles").scrollable();
    let panel_intersections = Panel::new(select_intersection)
        .title("Interseccion")
        .scrollable();

    //siv.add_layer(select_line);
    siv.add_layer(
        linear_layout
            .child(panel)
            .child(panel_streets)
            .child(panel_intersections),
    );
}

fn handle_street_select(s: &mut Cursive, street_id: &String) -> Result<(), Box<dyn Error>> {
    let streets = bus_lines::get_streets(RequestAction::Intersection {
        line: 
    }, params);

    Ok(())
}

fn handle_line_select(s: &mut Cursive, line_id: &String) -> Result<(), Box<dyn Error>> {
    let streets = bus_lines::get_streets(
        &bus_lines::RequestAction::Street(&Line {
            line_id: line_id.to_owned(),
            name: "FRanco".to_string(),
        }),
        HashMap::new(),
    )
    .unwrap();

    s.call_on_name("select_street", |view: &mut SelectView<String>| {
        view.add_all(
            streets
                .iter()
                .map(|street| (street.desc.to_string(), street.id.to_string())),
        );
    });
    s.focus_name("select_street").unwrap();

    Ok(())
}
