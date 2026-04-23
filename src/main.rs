use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::Print,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;
use std::{time::Duration, vec};

type charTab = Vec<Vec<char>>;

const SOK: char = '💂';
const CAISSE: char = '📦';
const MUR: char = '⬛';

struct Pos {
    x: i32,
    y: i32,
}

struct Game {
    map: charTab,
    sokPos: Pos,
    victoire: bool,
}

fn main() {
    execute!(stdout(), EnterAlternateScreen).unwrap();
    let _ = enable_raw_mode();
    let mut jeu = Game::init();
    jeu.show();
    loop {
        if let Ok(k) = key_pressed() {
            match k {
                'z' => jeu.MoveSoko(&0, &-1),
                's' => jeu.MoveSoko(&0, &1),
                'd' => jeu.MoveSoko(&1, &0),
                'q' => jeu.MoveSoko(&-1, &0),
                _ => (),
            }
        }
    }
    execute!(stdout(), LeaveAlternateScreen).unwrap();
}

fn key_pressed() -> Result<char, bool> {
    if event::poll(Duration::from_millis(100)).unwrap_or(false) {
        if let Ok(Event::Key(key_pressed)) = event::read()
            && key_pressed.kind == KeyEventKind::Press
        {
            match key_pressed.code {
                KeyCode::Char('z') => Ok('z'),
                KeyCode::Char('s') => Ok('s'),
                KeyCode::Char('d') => Ok('d'),
                KeyCode::Char('q') => Ok('q'),
                _ => Err(false),
            }
        } else {
            Err(false)
        }
    } else {
        Err(false)
    }
}

impl Game {
    fn init() -> Self {
        let mut map_init = vec![vec![' '; 15]; 15];
        let pos_init = Pos { x: 1, y: 1 };
        let pos_caisses = vec![
            Pos { x: 1, y: 2 },
            Pos { x: 10, y: 10 },
            Pos { x: 6, y: 14 },
        ];

        // creation map
        map_init[pos_init.x as usize][pos_init.y as usize] = SOK;
        for caisse in pos_caisses {
            map_init[caisse.x as usize][caisse.y as usize] = CAISSE;
        }

        for i in 0..15 {
            for j in 0..14 {
                map_init[0][j] = MUR;
                map_init[14][j] = MUR;
            }
            map_init[i][0] = MUR;
            map_init[i][14] = MUR;
        }

        // On rend le self
        Self {
            map: map_init,
            sokPos: pos_init,
            victoire: false,
        }
    }

    fn show(&self) {
        for ligne in &self.map {
            for case in ligne {
                if case == &' ' {
                    print!("{:2}", case);
                } else {
                    print!("{}", case);
                }
            }
            println!();
        }
    }

    fn MoveSoko(&mut self, dep_x: &i32, dep_y: &i32) {
        self.sokPos.x += dep_x;
        self.sokPos.y += dep_y;
        draw_at((self.sokPos.x) as u16, (self.sokPos.y + 9) as u16, SOK);
    }
}

fn draw_at(x: u16, y: u16, c: char) {
    execute!(stdout(), cursor::MoveTo(x, y), Print(c)).unwrap();
}
