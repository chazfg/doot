use crate::BorshDeserialize;
use crate::task_list::TaskIndexer;
use crate::task_status::TaskStatus;
use borsh::BorshSerialize;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;

#[derive(Default, BorshSerialize, BorshDeserialize, Clone)]
pub struct Task {
    title: String,
    status: TaskStatus,
    desc: String,
    children: Vec<Task>,
}

impl Task {
    pub fn incr_level(&self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::Null | TaskIndexer::Selected => panic!(),
            TaskIndexer::Child(selected_child_index, task_indexer) => {
                self.children[*selected_child_index].incr_level(task_indexer);
            }
            TaskIndexer::SelectedChild(selected_child_index) => {
                if self.children[*selected_child_index].child_count() != 0 {
                    *task_indexer = TaskIndexer::Child(
                        *selected_child_index,
                        Box::new(TaskIndexer::SelectedChild(0)),
                    );
                }
            }
        }
    }

    pub fn decr_level(&self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::Null | TaskIndexer::Selected => panic!(),
            TaskIndexer::Child(child_index, child_task_indexer) => match &**child_task_indexer {
                TaskIndexer::SelectedChild(_) => {
                    **child_task_indexer = TaskIndexer::Selected;
                    *task_indexer = TaskIndexer::SelectedChild(*child_index);
                }
                TaskIndexer::Child(_, _) => {
                    self.children[*child_index].decr_level(child_task_indexer);
                }
                TaskIndexer::Null | TaskIndexer::Selected => panic!(),
            },
            TaskIndexer::SelectedChild(selected_child_index) => {
                if self.children[*selected_child_index].child_count() != 0 {
                    *task_indexer = TaskIndexer::Child(
                        *selected_child_index,
                        Box::new(TaskIndexer::SelectedChild(0)),
                    );
                }
            }
        }
    }

    pub fn delete_task(&mut self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::Null | TaskIndexer::Selected => panic!(),
            TaskIndexer::Child(selected_child_index, child_task_indexer) => {
                match **child_task_indexer {
                    TaskIndexer::Null | TaskIndexer::Selected => (),
                    TaskIndexer::Child(_, _) => {
                        self.children[*selected_child_index].delete_task(child_task_indexer)
                    }
                    TaskIndexer::SelectedChild(child_selected_child_index) => {
                        self.children[*selected_child_index]
                            .children
                            .remove(child_selected_child_index);
                        if self.children[*selected_child_index].child_count() == 0 {
                            *task_indexer = TaskIndexer::SelectedChild(*selected_child_index);
                        }
                    }
                }
            }
            TaskIndexer::SelectedChild(selected_child_index) => {
                self.children.remove(*selected_child_index);
                if *selected_child_index == self.children.len() {
                    *selected_child_index -= 1;
                }
            }
        };
    }

    pub fn new(title: String, status: TaskStatus, desc: String) -> Self {
        Self {
            title,
            status,
            desc,
            children: vec![],
        }
    }

    pub fn count(&self) -> u16 {
        1 + self.children.iter().map(|t| t.count()).sum::<u16>()
    }

    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    pub fn counts_vec(&self) -> Vec<u16> {
        self.children.iter().map(|c| c.count()).collect()
    }

    pub fn decr(&mut self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(selected_child_index) => {
                *selected_child_index = (*selected_child_index + 1) % self.children.len();
            }
            TaskIndexer::Child(selected_child_index, task_indexer) => {
                self.children[*selected_child_index].decr(task_indexer);
            }
            TaskIndexer::Selected => todo!(),
            TaskIndexer::Null => todo!(),
        }
    }

    pub fn incr(&mut self, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(selected_child_index) => {
                if *selected_child_index != 0 {
                    *selected_child_index -= 1;
                } else if !self.children.is_empty() {
                    *selected_child_index = self.children.len() - 1;
                }
            }
            TaskIndexer::Child(selected_child_index, task_indexer) => {
                self.children[*selected_child_index].incr(task_indexer);
            }
            TaskIndexer::Selected => todo!(),
            TaskIndexer::Null => todo!(),
        }
    }

    pub fn draw(
        &self,
        frame: &mut Frame,
        paintable_area: &Rect,
        child_offset: i32,
        selected_task: &TaskIndexer,
    ) {
        let mut v_splits = vec![Constraint::Length(4)];
        self.counts_vec().iter().for_each(|c| {
            v_splits.push(Constraint::Length(4 * c));
        });

        let task_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(v_splits)
            .split(*paintable_area);

        let this_task_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(child_offset as u16), Constraint::Fill(1)])
            .split(task_rows[0]);

        match selected_task {
            TaskIndexer::SelectedChild(selected_child_index) => {
                let block_para = self.simple_block();
                frame.render_widget(block_para, this_task_row[1]);
                self.children
                    .iter()
                    .zip(task_rows[1..].iter())
                    .enumerate()
                    .for_each(|(i, (child_task, child_layout))| {
                        if i != *selected_child_index {
                            child_task.draw(
                                frame,
                                child_layout,
                                child_offset + 4,
                                &TaskIndexer::Null,
                            );
                        } else {
                            child_task.draw(
                                frame,
                                child_layout,
                                child_offset + 4,
                                &TaskIndexer::Selected,
                            );
                        }
                    });
            }
            TaskIndexer::Child(selected_child_index, task_indexer) => {
                let block_para = self.simple_block();
                frame.render_widget(block_para, this_task_row[1]);
                self.children
                    .iter()
                    .zip(task_rows[1..].iter())
                    .enumerate()
                    .for_each(|(i, (child_task, child_layout))| {
                        if i != *selected_child_index {
                            child_task.draw(
                                frame,
                                child_layout,
                                child_offset + 4,
                                &TaskIndexer::Null,
                            );
                        } else {
                            child_task.draw(frame, child_layout, child_offset + 4, task_indexer);
                        }
                    });
            }
            TaskIndexer::Selected => {
                let block_para = self.selected_block();
                frame.render_widget(block_para, this_task_row[1]);
                self.children.iter().zip(task_rows[1..].iter()).for_each(
                    |(child_task, child_layout)| {
                        child_task.draw(frame, child_layout, child_offset + 4, &TaskIndexer::Null);
                    },
                );
            }
            TaskIndexer::Null => {
                let block_para = self.simple_block();
                frame.render_widget(block_para, this_task_row[1]);
                self.children.iter().zip(task_rows[1..].iter()).for_each(
                    |(child_task, child_layout)| {
                        child_task.draw(frame, child_layout, child_offset + 4, &TaskIndexer::Null);
                    },
                );
            }
        }
    }

    fn simple_block(&self) -> Paragraph {
        Paragraph::new(&*self.desc).block(
            Block::bordered()
                .title(&*self.title)
                .title(self.status.to_line()),
        )
    }

    fn selected_block(&self) -> Paragraph {
        Paragraph::new(&*self.desc).block(
            Block::bordered()
                .title(&*self.title)
                .title(self.status.to_line())
                .border_style(Color::Cyan),
        )
    }

    pub fn append_task(&mut self, new_task: Task, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(_) => self.children.push(new_task),
            TaskIndexer::Child(child_index, child_task_indexer) => {
                self.children[*child_index].append_task(new_task, child_task_indexer)
            }
            TaskIndexer::Selected | TaskIndexer::Null => panic!(),
        }
    }

    pub fn prepend_task(&mut self, new_task: Task, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(_) => self.children.insert(0, new_task),
            TaskIndexer::Child(child_index, child_task_indexer) => {
                self.children[*child_index].prepend_task(new_task, child_task_indexer)
            }
            TaskIndexer::Selected | TaskIndexer::Null => panic!(),
        }
    }

    pub fn insert_task_above(&mut self, new_task: Task, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(selected_child_index) => {
                self.children.insert(*selected_child_index, new_task)
            }
            TaskIndexer::Child(child_index, child_task_indexer) => {
                self.children[*child_index].insert_task_above(new_task, child_task_indexer)
            }
            TaskIndexer::Selected | TaskIndexer::Null => panic!(),
        }
    }

    pub fn insert_task_below(&mut self, new_task: Task, task_indexer: &mut TaskIndexer) {
        match task_indexer {
            TaskIndexer::SelectedChild(selected_child_index) => {
                if *selected_child_index != self.children.len() - 1 {
                    self.children.insert((*selected_child_index) + 1, new_task)
                } else {
                    self.children.push(new_task)
                }
            }
            TaskIndexer::Child(child_index, child_task_indexer) => {
                self.children[*child_index].insert_task_below(new_task, child_task_indexer)
            }
            TaskIndexer::Selected | TaskIndexer::Null => panic!(),
        }
    }

    pub fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
    }
}
