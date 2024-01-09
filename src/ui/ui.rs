use crate::todo::todo::Todo;
use crossterm::{
    cursor,
    event::{self, Event},
    execute, queue,
    style::{self, Stylize},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};

pub struct TodoUI {
    todo: Todo,
    cols: u16,
    rows: u16,
    stdout: io::Stdout,
    selected_task: usize,
}

macro_rules! key_press {
    ($key:expr) => {
        Event::Key(event::KeyEvent {
            code: event::KeyCode::Char($key),
            kind: event::KeyEventKind::Press,
            ..
        })
    };
    ($key:expr, $modifier:expr) => {
        Event::Key(event::KeyEvent {
            code: event::KeyCode::Char($key),
            modifiers: $modifier,
            kind: event::KeyEventKind::Press,
            ..
        })
    };
}

impl TodoUI {
    pub fn new(todo: Todo) -> Self {
        let (cols, rows) = terminal::size().expect("Failed to get terminal size");
        Self {
            todo,
            cols,
            rows,
            stdout: io::stdout(),
            selected_task: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            Clear(ClearType::All),
            cursor::Hide
        )?;
        terminal::enable_raw_mode().expect("Failed to enable raw mode");

        self.loop_ui()?;

        terminal::disable_raw_mode().expect("Failed to disable raw mode");
        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            Clear(ClearType::All),
            cursor::Show
        )?;

        Ok(())
    }

    fn handle_input(&mut self) -> Result<(), io::Error> {
        match event::read()? {
            key_press!('q') => {
                std::process::exit(0);
            }
            key_press!('j') => {
                if self.selected_task < self.todo.get_all_tasks().len() - 1 {
                    self.selected_task += 1;
                }

                Ok(())
            }
            key_press!('k') => {
                if self.selected_task > 0 {
                    self.selected_task -= 1;
                }

                Ok(())
            }
            Event::Resize(cols, rows) => {
                self.cols = cols;
                self.rows = rows;

                execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;

                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn draw(&mut self) -> Result<(), io::Error> {
        // draw todos
        let tasks = self.todo.get_all_tasks();

        let mut cumulative_overflow = 0u16;
        for (index, task) in tasks.iter().enumerate() {
            let task_text = match task.done {
                true => format!(
                    "{}{}{} {}",
                    "- [".dark_grey(),
                    "✓".green(),
                    "]".dark_grey(),
                    task.description
                        .clone()
                        .on(match self.selected_task == index as usize {
                            true => style::Color::Grey,
                            false => style::Color::Black,
                        })
                        .with(match self.selected_task == index as usize {
                            true => style::Color::Black,
                            false => style::Color::DarkGrey,
                        })
                ),
                false => format!(
                    "- [ ] {}",
                    task.description
                        .clone()
                        .on(match self.selected_task == index as usize {
                            true => style::Color::Grey,
                            false => style::Color::Black,
                        })
                        .with(match self.selected_task == index as usize {
                            true => style::Color::Black,
                            false => style::Color::White,
                        })
                ),
            };

            queue!(
                self.stdout,
                cursor::MoveTo(0, index as u16 + cumulative_overflow),
                terminal::Clear(terminal::ClearType::CurrentLine),
                style::PrintStyledContent(task_text.clone().stylize())
            )?;

            for line in task_text.lines() {
                cumulative_overflow += line.len() as u16 / self.cols;
            }
        }

        self.stdout.flush()?;

        Ok(())
    }

    fn loop_ui(&mut self) -> Result<(), io::Error> {
        loop {
            self.handle_input()?;
            self.draw()?;
        }
    }
}
