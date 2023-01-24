//Includes
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, stdout, stdin, Write},
    path::Path
};

use termion::{
    event::Key,
    input::TermRead,
    raw::IntoRawMode
};

use colored::Colorize;

struct Cursor {
    x: usize,
    y: usize,
    lines: Vec<String>
}

//Entry point of program
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = parse_args(&args); 

    let mut lines = lines_from_file(filename).unwrap();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    if lines.len() == 0 {
        lines.push(String::new());
    }
    let mut cursor = Cursor {x: lines[lines.len() - 1].len() + 1, y: lines.len(), lines: lines};
    display_new_file(&cursor.lines);

    for c in stdin.keys() {
        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Esc => break,
            Key::Left      => cursor.left(),
            Key::Right     => cursor.right(),
            Key::Up        => cursor.up(),
            Key::Down      => cursor.down(),
            Key::Char('\n') => cursor.newline(),
            Key::Backspace      => cursor.delete(),
            Key::Char('\t') => cursor.tab(),
            Key::Char(c)   => cursor.insert(c),
            _              => panic!("Unknown crashing!")
            
        }

        // Flush again
        stdout.flush().unwrap();
        display_new_file(&cursor.lines);
        write!(stdout, "{}", termion::cursor::Goto(cursor.x as u16, cursor.y as u16)).unwrap();
        stdout.flush().unwrap();
    }

    clearscreen::clear().expect("failed to clear screen");

    let mut file = File::create(filename).expect("Yes");
    for i in cursor.lines {
        file.write_all(i.as_ref()).expect("Yes");
        file.write_all("\n".as_ref()).expect("Yes");
    }

}

//Determine if arguments are valid
fn parse_args(args: &Vec<String>) -> &String
{
    if args.len() != 2 {
        panic!("Usage: 'jot <filename.txt>'");
    }
    &args[1]
}

//Get lines from the file in a vector of strings
fn lines_from_file(filename: &String) -> io::Result<Vec<String>> {
    if !Path::new(filename).exists() {
        File::create(filename).expect("Failed to create file!");
    }
    BufReader::new(File::open(filename)?).lines().collect()
}

//Clear output and display new file
fn display_new_file(lines: &Vec<String>) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Show, termion::cursor::Goto(0, 1)).unwrap();

    let mut i = 0;
    for line in lines {
        if i != lines.len() - 1 {
            write!(stdout, "{}\n\r", line.green().bold()).expect("Failed to write line!");
        }
        else {
            write!(stdout, "{}", line.green().bold()).expect("Failed to write line!");
        }

        i +=1;
    }

    stdout.flush().unwrap();
}

impl Cursor {
    pub fn up(&mut self) {
        if self.y - 1 == 0 {
           return
        }
        self.y -= 1;

        if self.x > self.lines[self.y - 1].len() + 1 {
            self.x = self.lines[self.y - 1].len() + 1;
        }
    }

    pub fn down(&mut self) {
        if self.y == self.lines.len() {
            return
        }
        self.y += 1;

        if self.x > self.lines[self.y - 1].len() + 1 {
            self.x = self.lines[self.y - 1].len() + 1;
        }
    }

    pub fn left(&mut self) {
        if self.x != 0 {
            self.x -= 1;
        }
    }

    pub fn right(&mut self) {
        self.x += 1;

        if self.x > self.lines[self.y - 1].len() + 1 {
            self.x = self.lines[self.y - 1].len() + 1;
        }
    }

    pub fn insert(&mut self, c: char)
    {
        if self.x == 0 {
            self.x = 1;
        }
        self.lines[self.y - 1].insert(self.x - 1, c);
        self.right();
    }

    pub fn delete(&mut self)
    {
        if self.y >= 1 {
            if self.x > self.lines[self.y - 1].len() + 1 {
                self.lines[self.y - 1].pop();
                self.left();
                return
            }
           
            else if self.x == 1 && self.y > 1 {
                self.x = self.lines[self.y - 2].len() + 1;
                let together = format!("{}{}", self.lines[self.y - 2], self.lines[self.y - 1]);
                self.lines.remove(self.y - 1);
                self.lines.remove(self.y - 2);
                self.lines.insert(self.y - 2, together);
                self.up();
                
            }

            else if self.x > 1 {
                self.lines[self.y - 1].remove(self.x - 2);
                self.left();
                return
            }
        }

    }

    pub fn newline(&mut self)
    {
        if self.lines[self.y-1].is_empty() {
            self.lines.insert(self.y, String::new());
            self.down();
            return
        }

        if self.x == 0 {
            self.x = 1;
        }
        let slice = &self.lines[self.y-1][(self.x-1)..self.lines[self.y-1].len()];
        self.lines.insert(self.y, String::from(slice));
        let i = self.lines[self.y-1].len();
        self.lines[self.y-1].replace_range((self.x-1)..i, "");
        self.down();
        self.x = 1;
    }

    pub fn tab(&mut self)
    {
        self.insert(' ');
        self.insert(' ');
        self.insert(' ');
        self.insert(' ');
    }
}
