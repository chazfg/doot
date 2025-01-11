use std::fs::OpenOptions;

use crate::Task;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use ratatui::prelude::*;

#[derive(Default, BorshSerialize, BorshDeserialize)]
pub struct TaskList(Vec<Task>);

impl TaskList {
    pub fn save(self, file: &str) {
        let mut file = OpenOptions::new().write(true).open(file).unwrap();
        self.serialize(&mut file).unwrap();
    }

    pub fn incr_level(&self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::Child(selected_child_index, task_indexer) => {
                self.0[*selected_child_index].incr_level(task_indexer);
            }
            TaskIndexer::SelectedChild(selected_child_index) => {
                if self.0[*selected_child_index].child_count() != 0 {
                    *task_indexer = TaskIndexer::Child(
                        *selected_child_index,
                        Box::new(TaskIndexer::SelectedChild(0)),
                    );
                }
            }
            TaskIndexer::Null => (),
            TaskIndexer::Selected => panic!(),
        }
    }

    pub fn decr_level(&self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::Child(child_index, child_task_indexer) => match &**child_task_indexer {
                TaskIndexer::SelectedChild(_) => {
                    **child_task_indexer = TaskIndexer::Selected;
                    *task_indexer = TaskIndexer::SelectedChild(*child_index);
                }
                TaskIndexer::Child(_, _) => {
                    self.0[*child_index].decr_level(child_task_indexer);
                }
                TaskIndexer::Null | TaskIndexer::Selected => panic!(),
            },
            TaskIndexer::SelectedChild(_selected_child_index) => (),
            TaskIndexer::Null => (),
            TaskIndexer::Selected => panic!(),
        }
    }

    pub fn delete_task(&mut self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::Null | TaskIndexer::Selected => (),
            TaskIndexer::Child(selected_child_index, child_task_indexer) => {
                match **child_task_indexer {
                    TaskIndexer::Null | TaskIndexer::Selected => (),
                    TaskIndexer::Child(_, _) => {
                        self.0[*selected_child_index].delete_task(child_task_indexer)
                    }
                    TaskIndexer::SelectedChild(child_selected_child_index) => {
                        self.0[*selected_child_index].remove_child(child_selected_child_index);
                        if self.0[*selected_child_index].child_count() == 0 {
                            *task_indexer = TaskIndexer::SelectedChild(*selected_child_index);
                        }
                    }
                }
            }
            TaskIndexer::SelectedChild(selected_child_index) => {
                self.0.remove(*selected_child_index);

                if self.0.is_empty() {
                    *task_indexer = TaskIndexer::Null;
                } else if *selected_child_index == self.0.len() {
                    *selected_child_index -= 1;
                }
            }
        };
    }

    pub fn append_task(&mut self, new_task: Task, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(_) => self.0.push(new_task),
            TaskIndexer::Child(child_index, child_task_indexer) => {
                self.0[*child_index].append_task(new_task, child_task_indexer)
            }
            TaskIndexer::Null => {
                self.0.push(new_task);
                *task_indexer = TaskIndexer::SelectedChild(0);
            }
            TaskIndexer::Selected => panic!(),
        }
    }

    pub fn prepend_task(&mut self, new_task: Task, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(_) => self.0.insert(0, new_task),
            TaskIndexer::Child(child_index, child_task_indexer) => {
                self.0[*child_index].prepend_task(new_task, child_task_indexer)
            }
            TaskIndexer::Null => {
                self.0.push(new_task);
                *task_indexer = TaskIndexer::SelectedChild(0);
            }
            TaskIndexer::Selected => panic!(),
        }
    }

    pub fn insert_task_above(&mut self, new_task: Task, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(selected_child_index) => {
                self.0.insert(*selected_child_index, new_task)
            }
            TaskIndexer::Child(child_index, child_task_indexer) => {
                self.0[*child_index].insert_task_above(new_task, child_task_indexer)
            }
            TaskIndexer::Null => {
                self.0.push(new_task);
                *task_indexer = TaskIndexer::SelectedChild(0);
            }
            TaskIndexer::Selected => panic!(),
        }
    }

    pub fn insert_task_below(&mut self, new_task: Task, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(selected_child_index) => {
                if *selected_child_index != self.0.len() - 1 {
                    self.0.insert((*selected_child_index) + 1, new_task)
                } else {
                    self.0.push(new_task)
                }
            }
            TaskIndexer::Child(child_index, child_task_indexer) => {
                self.0[*child_index].insert_task_below(new_task, child_task_indexer)
            }
            TaskIndexer::Null => {
                self.0.push(new_task);
                *task_indexer = TaskIndexer::SelectedChild(0);
            }
            TaskIndexer::Selected => panic!(),
        }
    }

    pub fn default_bytes() -> Vec<u8> {
        let mut buf = Vec::new();
        let default_list = Self::default();
        let _ = default_list.serialize(&mut buf);
        buf
    }
    pub fn draw(&self, frame: &mut Frame, paintable_area: Rect, selected_task: &TaskIndexer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.counts_vec()
                    .into_iter()
                    .map(|i| Constraint::Length(i * 4))
                    .collect::<Vec<Constraint>>(),
            )
            .split(paintable_area);

        match selected_task {
            TaskIndexer::Selected => panic!(),
            TaskIndexer::SelectedChild(selected_child_index) => {
                self.0.iter().zip(layout.iter()).enumerate().for_each(
                    |(i, (task, task_layout))| {
                        if i != *selected_child_index {
                            task.draw(frame, task_layout, 0, &TaskIndexer::Null);
                        } else {
                            task.draw(frame, task_layout, 0, &TaskIndexer::Selected);
                        }
                    },
                );
            }
            TaskIndexer::Child(selected_child, child_task_indexer) => self
                .0
                .iter()
                .zip(layout.iter())
                .enumerate()
                .for_each(|(i, (task, task_layout))| {
                    if i != *selected_child {
                        task.draw(frame, task_layout, 0, &TaskIndexer::Null);
                    } else {
                        task.draw(frame, task_layout, 0, child_task_indexer);
                    }
                }),
            TaskIndexer::Null => {
                self.0
                    .iter()
                    .zip(layout.iter())
                    .for_each(|(task, task_layout)| {
                        task.draw(frame, task_layout, 0, &TaskIndexer::Null);
                    })
            }
        };
    }

    pub fn counts_vec(&self) -> Vec<u16> {
        self.0.iter().map(|t| t.count()).collect()
    }

    pub fn first_task(&self) -> TaskIndexer {
        if !self.0.is_empty() {
            TaskIndexer::SelectedChild(0)
        } else {
            TaskIndexer::Null
        }
    }

    pub fn decr(&mut self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(selected_child_index) => {
                *selected_child_index = (*selected_child_index + 1) % self.0.len();
            }
            TaskIndexer::Child(selected_child_index, task_indexer) => {
                self.0[*selected_child_index].decr(task_indexer);
            }
            TaskIndexer::Null => (),
            TaskIndexer::Selected => todo!(),
        }
    }

    pub fn incr(&mut self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(selected_child_index) => {
                if *selected_child_index != 0 {
                    *selected_child_index -= 1;
                } else if !self.0.is_empty() {
                    *selected_child_index = self.0.len() - 1;
                }
            }
            TaskIndexer::Child(selected_child_index, task_indexer) => {
                self.0[*selected_child_index].incr(task_indexer);
            }
            TaskIndexer::Null => (),
            TaskIndexer::Selected => todo!(),
        }
    }
}

pub enum TaskIndexer {
    SelectedChild(usize), // the level this is matched means it's child is selected
    Child(usize, Box<TaskIndexer>), // this will pull as which to pass the child indexer to
    Selected,             // whatever task gets passed this is the one that is selected
    Null,
}
