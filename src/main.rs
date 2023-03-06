use crossterm::cursor::{
    MoveTo, MoveToNextLine, MoveToPreviousLine, RestorePosition, SavePosition,
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

fn main() -> Result<()> {
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

    // init render, first frame
    render_setup()?;
    render_todos(&todos)?;

    loop {
        if poll(Duration::from_millis(100))? {

            let pos = position()?;
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
                        todos.remove(pos.1 as usize - 8);
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

            execute!(stdout, SavePosition)?;

            // render
            render_setup()?;
            render_todos(&todos)?;

            execute!(stdout, RestorePosition)?;
        }
    }

    disable_raw_mode()?;

    Ok(())
}
