use crossterm::{event, execute, terminal, cursor, queue};
use std::{cmp, env, fs, io};
use std::io::{Read, stdout, Write};
use std::path::Path;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::ClearType;

mod bf;
mod b_unf;

/* Followed tutorial at https://medium.com/@otukof/build-your-text-editor-with-rust-678a463f968b for basic setup
Github at https://github.com/Kofituo/pound/blob/highlight_digit/src/main.rs
currently at https://medium.com/@otukof/build-your-text-editor-with-rust-part-4-fd4a8b8641f8 */

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
        execute!(stdout(), terminal::Clear(ClearType::All)).expect("Clear failed");
    }
}

#[derive(Default)]
struct Editor{
    win_size: (usize, usize),
    cursor: (usize, usize),
    offset: (usize, usize),

    editor_contents: Vec<String>,
    display_contents: String,
}

impl Editor{

    fn new(text: Option<String>) -> Self{

        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();

        let editor_contents = match text{
            Some(text) => {text.lines().map(|ln|ln.into()).collect()},
            None => Vec::new()
        };

        Self { win_size,
            editor_contents,
            ..Default::default()}
    }

    fn clear_screen() -> io::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn read_key(&mut self) -> io::Result<KeyEvent>{

        loop {
            // Delay is here in case we want to do something between key presses
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {return Ok(event);}
                if let Event::Resize(x, y) = event::read()?{self.win_size = (x as usize, y as usize)}
            }
        }
    }

    fn update_screen(&mut self) -> io::Result<()> {
        queue!(self,
            cursor::Hide,
            cursor::MoveTo(0, 0))?;
        self.draw_rows()?;

        let cursor = (self.cursor.0 as u16, self.cursor.1 as u16);
        queue!(self,
            cursor::MoveTo(cursor.0, cursor.1),
            cursor::Show)?;
        self.flush()
    }

    fn draw_rows(&mut self) -> io::Result<()>{
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;

        // if the editor is empty display a welcome screen
        if self.editor_contents.is_empty(){

            Self::clear_screen()?;

            let offset = 3;

            self.display_contents.push_str(&"~\r\n".repeat(offset));

            let mut welcome = "Kat Editor --- Version 1.0".to_owned();
            welcome.truncate(screen_columns);
            let mut padding = (screen_columns - welcome.len()) / 2;
            if padding != 0 {
                self.display_contents.push('~');
                padding -= 1
            }
            (0..padding).for_each(|_| self.display_contents.push(' '));
            self.display_contents.push_str(&welcome);

            self.display_contents.push_str(&"\r\n".repeat(screen_rows-offset-1));

            return Ok(());
        }

        for text_line in self.offset.1..screen_rows+self.offset.1 {

            if text_line < self.editor_contents.len() {
                self.display_contents.push_str(&self.editor_contents[text_line].chars()/*Todo? .skip(self.offset.0)*/
                    .take(screen_columns).collect::<String>())
            } else {
                self.display_contents.push('~');
            }
            queue!(
                self,
                terminal::Clear(ClearType::UntilNewLine)
            )?;

            panic!();
            self.display_contents.push_str("\r\n");
        }

        Ok(())
    }

    fn process_keypress(&mut self) -> io::Result<bool> {

        match self.read_key()? {

            // quiting
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL, ..
            } => return Ok(false),

            //
            KeyEvent {
                code: direction
                    @ (KeyCode::Up | KeyCode::Down
                    | KeyCode::Left | KeyCode::Right
                    | KeyCode::Home | KeyCode::End),
                modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press | KeyEventKind::Repeat, ..
            } => self.move_cursor(direction),

            KeyEvent {
                code: KeyCode::PageUp,
                modifiers: KeyModifiers::NONE, ..
            } => (0..self.win_size.1).for_each(|_| {self.move_cursor(KeyCode::Up);}),

            KeyEvent {
                code: KeyCode::PageDown,
                modifiers: KeyModifiers::NONE, ..
            } => (0..self.win_size.1).for_each(|_| {self.move_cursor(KeyCode::Down);}),

            _ => {}
        }
        Ok(true)
    }

    fn move_cursor(&mut self, direction: KeyCode) {

        match direction {
            KeyCode::Up => {

                if self.cursor.1 == 0 {
                    self.offset = (self.offset.0, self.offset.1.saturating_sub(1))
                }
                self.cursor.1 = self.cursor.1.saturating_sub(1);
            }
            KeyCode::Left => {
                self.cursor.0 = self.cursor.0.saturating_sub(1);
            }
            KeyCode::Down => {
                if self.cursor.1 != self.win_size.0 - 1 {
                    self.cursor.1 += 1;
                } else {
                    self.offset.1 += 1
                }
            }
            KeyCode::Right => {
                if self.cursor.0 != self.win_size.0 - 1 {
                    self.cursor.0 += 1;
                }
            }
            KeyCode::End => self.cursor.0 = self.win_size.0 - 1,
            KeyCode::Home => self.cursor.0 = 0,
            _ => unimplemented!(),
        }
    }

    fn run(&mut self) -> io::Result<bool> {
        self.update_screen()?;
        self.process_keypress()
    }
}

impl Write for Editor{

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.display_contents.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {

        let out = write!(stdout(), "{}", self.display_contents);
        stdout().flush()?;
        self.display_contents.clear(); // TODO Might have to change later
        out
    }
}

fn main() -> io::Result<()> {

    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;

    let mut arg = env::args();

    let file_contents= match arg.nth(1) {
        None => None,
        Some(file) => Some(fs::read_to_string(file).expect("Unable to read file"))
    };

    let mut editor = Editor::new(file_contents);
    while editor.run()? {}

    Ok(())
}
