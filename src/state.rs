use crate::Task;
use crate::buffered_task::BufferedTask;
use crossterm::event::KeyModifiers;

use crate::TaskList;
use crate::task_list::TaskIndexer;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::Block;

pub struct State {
    pub tasks: TaskList,
    pub selected_task: TaskIndexer,
}

impl State {
    pub fn incr_level(&mut self) {
        self.tasks.incr_level(&mut self.selected_task);
    }

    pub fn decr_level(&mut self) {
        self.tasks.decr_level(&mut self.selected_task);
    }

    pub fn handle_new_task(&mut self, new_task: Task, task_add_kind: KeyCode) {
        match task_add_kind {
            KeyCode::Char('a') => self.tasks.append_task(new_task, &mut self.selected_task),
            KeyCode::Char('A') => self.tasks.prepend_task(new_task, &mut self.selected_task),
            KeyCode::Char('I') => self
                .tasks
                .insert_task_above(new_task, &mut self.selected_task),
            KeyCode::Char('i') => self
                .tasks
                .insert_task_below(new_task, &mut self.selected_task),
            _ => (),
        };
    }

    pub fn delete_task(&mut self) {
        self.tasks.delete_task(&mut self.selected_task);
    }

    pub fn save(self, file: &str) {
        let Self { tasks, .. } = self;
        tasks.save(file);
    }

    pub fn draw(&self, frame: &mut Frame) {
        let app_block = Block::bordered().title_bottom(
            Line::from(vec![
                " Change Task ".into(),
                "<Up>/<Down>".green().bold(),
                " Change Level ".into(),
                "<Left>/<Right>".green().bold(),
                " Quit ".into(),
                "<Esc/q>".green().bold(),
                " Add Task (Start/End/Above/Below) ".into(),
                "<A/a/I/i>".green().bold(),
                " Delete Task ".into(),
                "<d> ".green().bold(),
            ])
            .centered(),
        );

        let paintable_area = app_block.inner(frame.area());

        frame.render_widget(app_block, frame.area());

        self.tasks.draw(frame, paintable_area, &self.selected_task);
    }

    pub fn incr(&mut self) {
        self.tasks.incr(&mut self.selected_task);
    }

    pub fn decr(&mut self) {
        self.tasks.decr(&mut self.selected_task);
    }

    pub fn add_task_loop<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Option<Task> {
        let mut buffered_task = BufferedTask::default();
        loop {
            let _ = terminal.draw(|frame| buffered_task.draw(frame));

            match event::read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => break buffered_task.save(),
                Event::Key(KeyEvent {
                    code: KeyCode::Up | KeyCode::BackTab,
                    ..
                }) => buffered_task.next_field(),
                Event::Key(KeyEvent {
                    code: KeyCode::Down | KeyCode::Enter | KeyCode::Tab,
                    ..
                }) => buffered_task.prev_field(),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => break None,
                Event::Key(KeyEvent {
                    code:
                        KeyCode::Char(c @ ('a'..='z' | '_' | '0'..='9' | ' ' | 'A'..='Z' | '?' | '!')),
                    ..
                }) => buffered_task.push_char(c),
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => buffered_task.right_arrow(),
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => buffered_task.left_arrow(),

                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => buffered_task.pop_char(),
                Event::Key(KeyEvent {
                    code: KeyCode::Delete,
                    ..
                }) => buffered_task.delete_char(),

                _ => (),
            }
        }
    }
}
