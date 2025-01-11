use ratatui::prelude::*;
#[derive(Default, Clone)]
pub struct TextInputField {
    left_buffer: String,
    _selection_buffer: String,
    right_buffer: String,
}

impl TextInputField {
    pub fn draw_unselected(&self, frame: &mut Frame, area: Rect) {
        let left_span = Span::styled(self.left_buffer.as_str(), Style::default());
        let right_span = Span::styled(self.right_buffer.as_str(), Style::default());
        let text_line = Line::from(vec![left_span, right_span]);

        frame.render_widget(text_line, area);
    }
    pub fn draw_selected(&self, frame: &mut Frame, area: Rect) {
        let left_span = Span::styled(self.left_buffer.as_str(), Style::default());
        let right_span = Span::styled(self.right_buffer.as_str(), Style::default());
        let left_span_width = left_span.width() as f64;
        let text_line = Line::from(vec![left_span, right_span]);

        frame.render_widget(text_line, area);
        frame.set_cursor_position((area.x + left_span_width as u16, area.y));
    }

    pub fn cursor_left(&mut self) {
        if let Some(char_to_right) = self.left_buffer.pop() {
            let mut new_right_buffer = char_to_right.to_string();
            let old_right_buffer = std::mem::take(&mut self.right_buffer);
            new_right_buffer.push_str(old_right_buffer.as_str());
            self.right_buffer = new_right_buffer;
        }
    }

    pub fn cursor_right(&mut self) {
        match self.right_buffer.len() {
            0 => (),
            1 => {
                let char_to_left = self.right_buffer.pop().unwrap();
                self.left_buffer.push(char_to_left);
            }
            2.. => {
                let old_right_buffer = std::mem::take(&mut self.right_buffer);
                let (char_to_left, new_right_buffer) = old_right_buffer.split_at(1);
                self.left_buffer.push_str(char_to_left);
                self.right_buffer = new_right_buffer.to_string();
            }
        }
    }

    pub fn push(&mut self, new_char: char) {
        self.left_buffer.push(new_char);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.left_buffer.pop()
    }

    pub fn delete_char(&mut self) {
        match self.right_buffer.len() {
            0 => (),
            1 => {
                self.right_buffer.pop();
            }
            2.. => {
                let old_right_buffer = std::mem::take(&mut self.right_buffer);
                let (_, new_right_buffer) = old_right_buffer.split_at(1);
                self.right_buffer = new_right_buffer.to_string();
            }
        }
    }

    pub fn return_buffer(self) -> String {
        let Self {
            mut left_buffer,
            right_buffer,
            ..
        } = self;
        left_buffer.push_str(right_buffer.as_str());
        left_buffer
    }
}
