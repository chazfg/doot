use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

#[derive(Default, BorshSerialize, BorshDeserialize, Clone)]
pub enum TaskStatus {
    Complete,
    InProgress,
    #[default]
    NotStarted,
}

impl TaskStatus {
    pub fn to_line(&self) -> Line {
        match self {
            Self::Complete => Line::from("Complete").right_aligned().style(Color::Green),
            Self::InProgress => Line::from("InProgress").right_aligned().style(Color::Blue),
            Self::NotStarted => Line::from("NotStarted").right_aligned().style(Color::Red),
        }
    }

    pub fn as_paragraph(&self) -> Paragraph {
        match self {
            Self::Complete => Paragraph::new("Complete").style(Style::new().green()),
            Self::InProgress => Paragraph::new("InProgress").style(Style::new().blue()),
            Self::NotStarted => Paragraph::new("NotStarted").style(Style::new().red()),
        }
    }

    pub fn as_paragraph_selected(&self) -> Paragraph {
        match self {
            Self::Complete => Paragraph::new("Complete").style(Style::new().light_green().bold()),
            Self::InProgress => {
                Paragraph::new("InProgress").style(Style::new().light_blue().bold())
            }
            Self::NotStarted => Paragraph::new("NotStarted").style(Style::new().light_red().bold()),
        }
    }

    pub fn next_status(&mut self) {
        *self = match self {
            TaskStatus::Complete => TaskStatus::NotStarted,
            TaskStatus::InProgress => TaskStatus::Complete,
            TaskStatus::NotStarted => TaskStatus::InProgress,
        }
    }

    pub fn prev_status(&mut self) {
        *self = match self {
            TaskStatus::Complete => TaskStatus::InProgress,
            TaskStatus::InProgress => TaskStatus::NotStarted,
            TaskStatus::NotStarted => TaskStatus::Complete,
        }
    }
}
