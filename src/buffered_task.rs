use crate::Task;
use crate::task_status::TaskStatus;
use crate::text_input::TextInputField;
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;

#[derive(Default)]
pub struct BufferedTask {
    title: TextInputField,
    desc: TextInputField,
    status: TaskStatus,
    selected_field: SelectedField,
}

impl BufferedTask {
    pub fn save(self) -> Option<Task> {
        let Self {
            title,
            desc,
            status,
            ..
        } = self;
        Some(Task::new(
            title.return_buffer(),
            status,
            desc.return_buffer(),
        ))
    }

    pub fn draw(&self, frame: &mut Frame) {
        let app_block = Block::bordered().title_bottom(
            Line::from(vec![
                " Change Field ".into(),
                "<Up>/<Down>".green().bold(),
                " Change Status ".into(),
                "<Left>/<Right>".green().bold(),
                " Cancel ".into(),
                "<ctrl+c>".green().bold(),
                " Save ".into(),
                "<ctrl+s> ".green().bold(),
            ])
            .centered(),
        );

        let paintable_area = app_block.inner(frame.area());

        let vertical = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(6),
            Constraint::Fill(1),
        ]);
        let [_title_area, vertical_main_area, _status_area] = vertical.areas(paintable_area);
        let main_horizontal = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(100),
            Constraint::Fill(1),
        ]);
        let [_left_half, main_area, _right_area] = main_horizontal.areas(vertical_main_area);

        let horizontal = Layout::horizontal([Constraint::Length(14), Constraint::Fill(1)]);
        let main_area_app = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]);

        let [title_edit, status_edit, desc_edit] = main_area_app.areas(main_area);

        let [title_left_area, title_right_area] = horizontal.areas(title_edit);
        let [status_left_area, status_right_area] = horizontal.areas(status_edit);
        let [desc_left_area, desc_right_area] = horizontal.areas(desc_edit);

        match self.selected_field {
            SelectedField::Title => (
                frame.render_widget(
                    Paragraph::new("-Title-").style(Style::default().bg(Color::DarkGray)),
                    title_left_area,
                ),
                frame.render_widget(
                    Paragraph::new(" Status ").style(Style::default().bg(Color::DarkGray)),
                    status_left_area,
                ),
                frame.render_widget(
                    Paragraph::new(" Description ").style(Style::default().bg(Color::DarkGray)),
                    desc_left_area,
                ),
            ),
            SelectedField::Desc => (
                frame.render_widget(
                    Paragraph::new(" Title ").style(Style::default().bg(Color::DarkGray)),
                    title_left_area,
                ),
                frame.render_widget(
                    Paragraph::new(" Status ").style(Style::default().bg(Color::DarkGray)),
                    status_left_area,
                ),
                frame.render_widget(
                    Paragraph::new("-Description-").style(Style::default().bg(Color::DarkGray)),
                    desc_left_area,
                ),
            ),
            SelectedField::Status => (
                frame.render_widget(
                    Paragraph::new(" Title ").style(Style::default().bg(Color::DarkGray)),
                    title_left_area,
                ),
                frame.render_widget(
                    Paragraph::new("-Status-").style(Style::default().bg(Color::DarkGray)),
                    status_left_area,
                ),
                frame.render_widget(
                    Paragraph::new(" Description ").style(Style::default().bg(Color::DarkGray)),
                    desc_left_area,
                ),
            ),
        };

        match self.selected_field {
            SelectedField::Title => {
                frame.render_widget(self.status.as_paragraph(), status_right_area);
                self.desc.draw_unselected(frame, desc_right_area);
                self.title.draw_selected(frame, title_right_area);
            }
            SelectedField::Status => {
                self.title.draw_unselected(frame, title_right_area);
                frame.render_widget(self.status.as_paragraph_selected(), status_right_area);
                self.desc.draw_unselected(frame, desc_right_area);
            }
            SelectedField::Desc => {
                self.title.draw_unselected(frame, title_right_area);
                frame.render_widget(self.status.as_paragraph(), status_right_area);
                self.desc.draw_selected(frame, desc_right_area);
            }
        };

        frame.render_widget(app_block, frame.area());
        // let area = centered_rect(60, 25, f.area());
    }

    pub fn next_field(&mut self) {
        self.selected_field = match self.selected_field {
            SelectedField::Title => SelectedField::Desc,
            SelectedField::Desc => SelectedField::Status,
            SelectedField::Status => SelectedField::Title,
        };
    }

    pub fn prev_field(&mut self) {
        self.selected_field = match self.selected_field {
            SelectedField::Title => SelectedField::Status,
            SelectedField::Desc => SelectedField::Title,
            SelectedField::Status => SelectedField::Desc,
        };
    }

    pub fn push_char(&mut self, new_char: char) {
        match self.selected_field {
            SelectedField::Title => self.title.push(new_char),
            SelectedField::Desc => self.desc.push(new_char),
            SelectedField::Status => (),
        }
    }

    pub fn pop_char(&mut self) {
        {
            match self.selected_field {
                SelectedField::Title => self.title.pop(),
                SelectedField::Desc => self.desc.pop(),
                SelectedField::Status => None,
            };
        }
    }

    pub fn delete_char(&mut self) {
        match self.selected_field {
            SelectedField::Title => self.title.delete_char(),
            SelectedField::Desc => self.desc.delete_char(),
            SelectedField::Status => (),
        }
    }

    pub fn right_arrow(&mut self) {
        match self.selected_field {
            SelectedField::Title => self.title.cursor_right(),
            SelectedField::Desc => self.desc.cursor_right(),
            SelectedField::Status => self.status.next_status(),
        }
    }

    pub fn left_arrow(&mut self) {
        match self.selected_field {
            SelectedField::Title => self.title.cursor_left(),
            SelectedField::Desc => self.desc.cursor_left(),
            SelectedField::Status => self.status.prev_status(),
        }
    }
}

#[derive(Default)]
enum SelectedField {
    #[default]
    Title,
    Desc,
    Status,
}
