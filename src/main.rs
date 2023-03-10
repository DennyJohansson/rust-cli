use crossterm::cursor::{
    MoveTo, MoveToNextLine, MoveToPreviousLine, MoveToRow, RestorePosition, SavePosition,
};
#[allow(unused)]
use crossterm::{
    cursor::position,
    event,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen, SetSize,
    },
    ExecutableCommand, Result,
};
#[allow(unused)]
use std::{io::stdout, time::Duration};

const HELP: &str = r#"Todo List
---- use Keyboard to navigate ----
 - hit "c" to create new todo
 - Hit "x" to delete tode at cursor
 - Hit "e" to edit todo at cursor
 - Hit "t" to toggle todo at cursor
 - Use Esc to quit
----------------------------------
"#;

struct Todo {
    text: String,
    completed: bool,
}

impl Todo {
    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

fn render_setup() -> Result<()> {
    execute!(
        stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0),
    )?;

    for line in HELP.lines() {
        execute!(stdout(), Print(line), MoveToNextLine(1))?;
    }

    Ok(())
}

fn render_todos(todos: &Vec<Todo>) -> Result<()> {
    let mut stdout = stdout();

    for todo in todos.iter() {
        if todo.completed {
            execute!(
                stdout,
                SetForegroundColor(Color::Red),
                Print("[x] "),
                ResetColor,
                Print(&todo.text),
                MoveToNextLine(1),
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::Green),
                Print("[ ] "),
                ResetColor,
                Print(&todo.text),
                MoveToNextLine(1),
            )?;
        }
    }

    Ok(())
}

enum State {
    Navigation,
    Create,
    Edit,
}

fn main() -> Result<()> {
    let mut state = State::Navigation;
    let mut stdout = stdout();
    let mut todos: Vec<Todo> = vec![
        Todo {
            text: "Buy milk".to_string(),
            completed: false,
        },
        Todo {
            text: "Buy eggs".to_string(),
            completed: true,
        },
        Todo {
            text: "Buy bread".to_string(),
            completed: false,
        },
        Todo {
            text: "Buy butter".to_string(),
            completed: true,
        },
    ];

    enable_raw_mode()?;

    render_setup()?;
    render_todos(&todos)?;

    loop {
        if poll(Duration::from_millis(100))? {
            let pos = position()?;
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => {
                        break;
                    }
                    e => {
                        match state {
                            State::Navigation => {
                                match e {
                                    KeyCode::Char('k') => {
                                        execute!(stdout, MoveToPreviousLine(1),)?;
                                    }
                                    KeyCode::Char('j') => {
                                        execute!(stdout, MoveToNextLine(1),)?;
                                    }
                                    KeyCode::Char('c') => {
                                        state = State::Create;
                                    }
                                    KeyCode::Char('x') => {
                                        todos.remove(pos.1 as usize - 8);
                                    }
                                    KeyCode::Char('e') => {
                                        state = State::Edit;
                                    }
                                    KeyCode::Char('t') => {
                                        let todo = &mut todos[pos.1 as usize - 8];

                                        todo.toggle();
                                    }
                                    _ => {}
                                }
                            }
                            State::Create => {
                                todos.push(Todo {
                                    text: "new todo".to_string(),
                                    completed: false,
                                });

                                execute!(stdout, MoveToRow(8 + todos.len() as u16))?;
                                state = State::Edit;
                            }
                            State::Edit => {
                                let todo = &mut todos[pos.1 as usize - 8];

                                match e {
                                    KeyCode::Char(c) => {
                                        todo.text.push(c);
                                    }
                                    KeyCode::Backspace => {
                                        todo.text.pop();
                                    }
                                    KeyCode::Enter => {
                                        state = State::Navigation;
                                    }
                                    KeyCode::Esc => {
                                        state = State::Navigation;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                },
                _ => {}
            }

            execute!(stdout, SavePosition)?;

            render_setup()?;
            render_todos(&todos)?;

            execute!(stdout, RestorePosition)?;
        }
    }

    disable_raw_mode()?;

    Ok(())
}
