mod input_mode;

use input_mode::InputMode;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

use crate::maze::{Maze, PositionRole};

pub struct App {
    input: String,
    cursor_position: usize,
    input_mode: InputMode,
    maze: Maze,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            cursor_position: 0,
            maze: Maze::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            maze: Maze::get_from_file(String::from("./Labirintos/100x100/exemplo_labirinto.txt")),
            ..App::default()
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_position)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let cursor_is_left_most = self.cursor_position == 0;

        if cursor_is_left_most {
            return;
        }

        let current_index = self.cursor_position;
        let from_left_to_current_index = current_index - 1;

        // Getting all characters before the selected character.
        let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
        // Getting all characters after selected character.
        let after_char_to_delete = self.input.chars().skip(current_index);

        self.input = before_char_to_delete.chain(after_char_to_delete).collect();

        self.move_cursor_left();
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn submit_message(&mut self) {
        // self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Writing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Writing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Writing => {}
                }
            }
        }
    }

    fn calculate_layout(area: Rect, division_size: usize) -> (Rect, Rect, Vec<Vec<Rect>>) {
        let main_layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ]);
        let block_layout = Layout::vertical(vec![Constraint::Max(4); division_size]);
        let [help_area, input_area, main_area] = main_layout.areas(area);
        let main_areas = block_layout
            .split(main_area)
            .iter()
            .map(|&area| {
                Layout::horizontal(vec![
                    Constraint::Percentage(100 / division_size as u16);
                    division_size
                ])
                .split(area)
                .to_vec()
            })
            .collect();
        (help_area, input_area, main_areas)
    }

    fn draw(&self, frame: &mut Frame) {
        let (help_area, input_area, main_areas) =
            App::calculate_layout(frame.area(), self.maze.size);

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start editing.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Writing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };

        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);

        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Writing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));

        frame.render_widget(input, input_area);

        match self.input_mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Normal => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Writing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + self.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            )),
        }

        let maze = self.maze.clone();

        // for i in 0..main_areas.len() {
        //     for j in 0..main_areas[i].len() {
        //         let area = main_areas[i][j];
        //         let background_color = maze.map[i][j].clone().into();

        //         App::render_block(frame, area, background_color);
        //     }
        // }
    }

    fn render_block(frame: &mut Frame, area: Rect, background_color: Color) {
        let block = Block::new()
            .border_style(Style::default().fg(Color::Magenta))
            .borders(Borders::NONE)
            .border_type(BorderType::Rounded)
            .bg(background_color);

        frame.render_widget(block, area);
    }
}
