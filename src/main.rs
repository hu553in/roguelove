use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

mod assets;
use assets::LOGO_TEXT;

#[derive(Debug, Clone, PartialEq)]
pub enum Scene {
    MainMenu,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Exit,
}

struct State {
    scene: Scene,
}

impl State {
    fn new() -> Self {
        Self {
            scene: Scene::MainMenu,
        }
    }
}

fn main() {
    let mut term = ratatui::init();
    let state = State::new();

    // main loop
    loop {
        // draw frame
        term.draw(|frame| draw(frame, &state))
            .expect("failed to draw frame");

        // handle input events
        if let Some(action) = handle_input_event(&state).expect("failed to handle input event") {
            match action {
                Action::Exit => break,
            }
        }
    }

    // restore terminal state
    ratatui::restore();
}

fn handle_input_event(state: &State) -> io::Result<Option<Action>> {
    match event::read()? {
        Event::Key(key_event) => Ok(handle_key_event(key_event, state)),
        _ => Ok(None),
    }
}

fn handle_key_event(key_event: KeyEvent, state: &State) -> Option<Action> {
    match key_event.code {
        KeyCode::Enter => {
            if state.scene == Scene::MainMenu {
                Some(Action::Exit)
            } else {
                None
            }
        }
        KeyCode::Esc | KeyCode::Char('q') => Some(Action::Exit),
        _ => None,
    }
}

fn draw(frame: &mut ratatui::Frame, state: &State) {
    match state.scene {
        Scene::MainMenu => {
            frame.render_widget(&*LOGO_TEXT, frame.area());
        }
    }
}
