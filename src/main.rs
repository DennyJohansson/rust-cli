use crossterm::cursor::{MoveTo, MoveToNextLine, MoveToPreviousLine, SavePosition, RestorePosition};
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

fn main() -> Result<()> {
    enable_raw_mode()?;

    // ---- setup ----
    execute!(
        stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0),
    )?;

    // ---- render help ----
    for line in HELP.lines() {
        execute!(stdout(), Print(line), MoveToNextLine(1))?;
    }

    // todo list
    // ---- events ----
    // - hit "c" to create new todo
    // - Hit "x" to delete tode at cursor
    // - Hit "e" to edit todo at cursor
    // - Hit "t" to toggle todo at cursor
    // - Use Esc to quit

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
    // ---- rendering ----
    loop {
        if poll(Duration::from_millis(100))? {
            // ---- events ----
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('k') => {
                        execute!(stdout, MoveToPreviousLine(1),)?;
                    }
                    KeyCode::Char('j') => {
                        execute!(stdout, MoveToNextLine(1),)?;
                    }
                    KeyCode::Char('c') => {
                        execute!(
                            stdout,
                            MoveTo(0, position()?.1),
                            Clear(ClearType::CurrentLine),
                            Print("create new todo"),
                        )?;
                    }
                    KeyCode::Char('x') => {
                        execute!(
                            stdout,
                            MoveTo(0, position()?.1),
                            Clear(ClearType::CurrentLine),
                            Print("delete todo")
                        )?;
                    }
                    KeyCode::Char('e') => {
                        execute!(
                            stdout,
                            MoveTo(0, position()?.1),
                            Clear(ClearType::CurrentLine),
                            Print("edit todo")
                        )?;
                    }
                    KeyCode::Char('t') => {
                        let pos = position()?;
                        let todo = &mut todos[pos.1 as usize - 8];

                        todo.toggle();
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
            // render todos
            execute!(stdout, SavePosition)?;
            execute!(stdout, MoveTo(0, 8))?;
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

            execute!(stdout, RestorePosition)?;
        }
    }

    disable_raw_mode()?;

    Ok(())
}