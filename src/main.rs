use borsh::BorshDeserialize;
use crossterm::event::KeyModifiers;
mod buffered_task;
mod state;
mod task;
mod task_list;
mod task_status;
mod text_input;
use crate::task_list::TaskList;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use state::State;
use task::Task;

fn main() {
    let tasks = match std::fs::File::open(".doot") {
        Ok(mut file) => match TaskList::try_from_reader(&mut file) {
            Ok(tasks) => tasks,
            Err(e) => panic!("{e:?}"),
        },
        Err(_) => match std::fs::write(".doot", TaskList::default_bytes()) {
            Ok(_new_file) => TaskList::default(),
            Err(e) => panic!("{e:?}"),
        },
    };
    load_terminal_interface(tasks);
}

fn load_terminal_interface(tasks: TaskList) {
    let mut terminal = ratatui::init();
    let selected_task = tasks.first_task();
    let mut application = State {
        tasks,
        selected_task,
    };

    loop {
        let _ = terminal.draw(|f| application.draw(f));

        match event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Esc,
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Up | KeyCode::BackTab,
                ..
            }) => application.incr(),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => application.incr_level(),
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => application.decr_level(),
            Event::Key(KeyEvent {
                code: KeyCode::Down | KeyCode::Tab,
                ..
            }) => application.decr(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                ..
            }) => application.delete_task(),
            Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break application.save(".doot"),
            // Event::Key(KeyEvent {
            // code: KeyCode::Char('e'),
            // ..
            // }) => application.edit_task_loop(&mut terminal),
            Event::Key(KeyEvent {
                /*
                a -> append
                A -> prepend
                I -> insert above
                i -> insert below
                */
                code:
                    task_add_kind @ (KeyCode::Char('a')
                    | KeyCode::Char('A')
                    | KeyCode::Char('i')
                    | KeyCode::Char('I')),
                ..
            }) => {
                if let Some(new_task) = application.add_task_loop(&mut terminal) {
                    application.handle_new_task(new_task, task_add_kind);
                }
            }

            _ => (),
        }
    }
    ratatui::restore();
}
