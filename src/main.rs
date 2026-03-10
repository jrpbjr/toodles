use color_eyre::eyre::{Ok, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, ListState,Paragraph, Padding, Widget},
    DefaultTerminal, Frame,
    text::{ToSpan},
};

#[derive(Debug, Default)]
struct  AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

enum FormAction{
    None,
    Submit,
    Escape,
}

fn main() -> Result<()>  {
    let mut state = AppState::default();
    state.is_add_new = false;

    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn  run (mut terminal: DefaultTerminal, app_state:&mut AppState) -> Result<()> {
    loop {
        //Rendering
        terminal.draw(|f|render(f, app_state))?;
        //Input handling
        if let Event::Key(key) = event::read()? {

            // evita repetir tecla
            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            if app_state.is_add_new{
                match  handle_add_new(key, app_state) {
                    FormAction::None => { },
                    FormAction::Submit => {
                        app_state.is_add_new = false;
                        app_state.items.push(TodoItem{
                            is_done: false,
                            description: app_state.input_value.clone(),
                        });
                        app_state.input_value.clear();
                    },
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_value.clear();
                    },
                }
            } else {
                if handle_key(key, app_state){
                    break;
                }
            }
        }
    }
    Ok(())
}

fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        },
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        event::KeyCode::Enter => {
            return FormAction::Submit;
        }
        event::KeyCode::Esc => {
            return FormAction::Escape
        }
        _ => {}
    }
    FormAction::None
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Enter => {
            if let Some(index) = app_state.list_state.selected() {
                if let Some(item) = &mut app_state.items.get_mut(index){
                    item.is_done = !item.is_done;
                }
            }
        }
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Char(char) => match char {
            'a' => {
                app_state.is_add_new = true;
            }
            'd' => {
                if let Some(index) = app_state.list_state.selected() {
                    app_state.items.remove(index);
                }
            }
            'k' => {
                app_state.list_state.select_previous();
            }
            'j' => {
                app_state.list_state.select_next();
            }
            _ => {}
        },
        _ => {}
    }
    false
}

fn render(frame: &mut Frame, app_state:&mut AppState){

    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    if app_state.is_add_new{
        render_input_form(app_state, border_area, frame);
    }else{
        render_list(border_area, frame, app_state);
    }

    fn render_input_form (app_state: &mut AppState, border_area: ratatui::prelude::Rect, frame: &mut Frame<'_>) {
        Paragraph::new(app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .title(" Input Description".to_span().into_centered_line())
                    .fg(Color::Green)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded),
            )
            .render(border_area, frame.buffer_mut());
    }

    fn render_list(border_area: ratatui::prelude::Rect,
                frame: &mut Frame<'_>,
                app_state: &mut AppState){

        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(" Toodles ".to_span().into_centered_line())
            .fg(Color::Yellow)
            .render(border_area, frame.buffer_mut());

        let list = List::new(
            app_state
                .items
                .iter()
                .map(|x| {
                    let value = if x.is_done {
                        x.description.to_span().underlined()
                    } else {
                        x.description.to_span().crossed_out()
                    };
                    ListItem::from(value)
                }),
        )
            .highlight_symbol(">")
            .highlight_style(Style::default().fg(Color::Green));

        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);

    }

}