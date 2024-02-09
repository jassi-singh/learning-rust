use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, queue,
    style::{self, Attribute, Attributes, Color, ResetColor, SetAttributes, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

struct Todo<'a> {
    title: &'a str,
    completed: bool,
}

impl Todo<'_> {
    fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

fn main() {
    let mut todo_list: Vec<Todo> = vec![
        Todo {
            title: "Go to market",
            completed: false,
        },
        Todo {
            title: "Go to gym",
            completed: true,
        },
    ];

    enable_raw_mode().unwrap();
    let mut curr_index: usize = 0;

    render_todo_list(&todo_list, curr_index);

    loop {
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up => {
                    curr_index = (curr_index + todo_list.len() - 1) % todo_list.len();
                }
                KeyCode::Down => {
                    curr_index = (curr_index + 1) % todo_list.len();
                }
                KeyCode::Char(' ') => {
                    todo_list.get_mut(curr_index).unwrap().toggle();
                }
                KeyCode::Esc => {
                    // Clear the screen before exiting
                    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
                    break;
                }
                _ => {}
            }

            render_todo_list(&todo_list, curr_index);
        }
    }

    disable_raw_mode().unwrap();
}

fn render_todo_list(todo_list: &Vec<Todo>, curr_index: usize) {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print("Esc to exit"),
        cursor::MoveToNextLine(1)
    )
    .unwrap();

    for (index, todo) in todo_list.iter().enumerate() {
        let status = if todo.completed { 'X' } else { ' ' };
        let message = format!("{:<2}. {:<30} [{status}]", index, todo.title);

        let attributes = if todo.completed {
            Attributes::from(&[Attribute::Dim, Attribute::CrossedOut][..])
        } else {
            Attributes::from(Attribute::Reset)
        };

        let color = if curr_index == index {
            Color::Green
        } else {
            Color::White
        };

        queue!(
            stdout(),
            SetAttributes(attributes),
            SetForegroundColor(color),
            style::Print(message),
            cursor::MoveToNextLine(1),
            SetAttributes(Attributes::from(Attribute::Reset)),
        )
        .unwrap();
    }

    stdout().flush().unwrap();
}
