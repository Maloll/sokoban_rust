use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::Print,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};
use std::time::Duration;

type CharTab = Vec<Vec<char>>;

const SOK: char = '💂';
const CAISSE: char = '📦';
const MUR: char = '⬛';
const CIBLE: char = '🔸';
const VIDE: char = ' ';

struct Pos {
    x: i32,
    y: i32,
}

struct Game {
    map: CharTab,
    sok_pos: Pos,
    pos_cibles: Vec<Pos>,
    victoire: bool,
}

fn main() {
    let mut out = stdout();
    execute!(out, EnterAlternateScreen).unwrap();
    let _ = enable_raw_mode();

    let mut jeu = Game::init();

    execute!(out, cursor::MoveTo(0, 0)).unwrap();

    jeu.show();

    loop {
        if let Ok(k) = key_pressed() {
            match k {
                'z' => jeu.MoveSoko(0, -1),
                's' => jeu.MoveSoko(0, 1),
                'd' => jeu.MoveSoko(1, 0),
                'q' => jeu.MoveSoko(-1, 0),
                'x' => break,
                _ => (),
            }

            cibles(&jeu);
            if victory(&jeu) {
                println!("VICTOIRE !");
                break;
            }
        }
    }

    let _ = disable_raw_mode();
    execute!(out, LeaveAlternateScreen).unwrap();
    jeu.show();
}

fn key_pressed() -> Result<char, bool> {
    if event::poll(Duration::from_millis(10)).unwrap_or(false) {
        if let Ok(Event::Key(key_pressed)) = event::read() {
            if key_pressed.kind == KeyEventKind::Press {
                return match key_pressed.code {
                    KeyCode::Char('z') => Ok('z'),
                    KeyCode::Char('s') => Ok('s'),
                    KeyCode::Char('d') => Ok('d'),
                    KeyCode::Char('q') => Ok('q'),
                    KeyCode::Char('x') => Ok('x'),
                    _ => Err(false),
                };
            }
        }
    }
    Err(false)
}

impl Game {
    fn init() -> Self {
        let mut map_init = vec![vec![VIDE; 15]; 15];
        let pos_init = Pos { x: 1, y: 1 };
        let pos_caisses = vec![
            Pos { x: 4, y: 2 },
            Pos { x: 10, y: 10 },
            Pos { x: 6, y: 12 },
        ];

        let position_cibles: Vec<Pos> =
            vec![Pos { x: 4, y: 5 }, Pos { x: 7, y: 7 }, Pos { x: 13, y: 6 }];

        for i in 0..15 {
            for j in 0..15 {
                if i == 0 || i == 14 || j == 0 || j == 14 {
                    map_init[i][j] = MUR;
                }
            }
        }

        for caisse in &pos_caisses {
            map_init[caisse.y as usize][caisse.x as usize] = CAISSE;
        }

        map_init[pos_init.y as usize][pos_init.x as usize] = SOK;
        for cible in &position_cibles {
            map_init[cible.y as usize][cible.x as usize] = CIBLE;
        }

        Self {
            map: map_init,
            sok_pos: pos_init,
            pos_cibles: position_cibles,
            victoire: false,
        }
    }

    fn show(&self) {
        for ligne in &self.map {
            for case in ligne {
                if *case == VIDE {
                    print!("  ");
                } else {
                    print!("{}", case);
                }
            }
            print!("\r\n");
        }
        let _ = stdout().flush();
    }

    fn MoveSoko(&mut self, dep_x: i32, dep_y: i32) {
        let old_x = self.sok_pos.x;
        let old_y = self.sok_pos.y;

        let x = old_x + dep_x;
        let y = old_y + dep_y;

        if x < 0 || x >= 15 || y < 0 || y >= 15 {
            return;
        }

        let cible = self.map[y as usize][x as usize];

        if cible == MUR {
            return;
        }

        if cible == CAISSE {
            let x_caisse = x + dep_x;
            let y_caisse = y + dep_y;

            if x_caisse < 0 || x_caisse >= 15 || y_caisse < 0 || y_caisse >= 15 {
                return;
            }

            let next_cible = self.map[y_caisse as usize][x_caisse as usize];
            if next_cible == MUR || next_cible == CAISSE {
                return;
            }

            self.map[y_caisse as usize][x_caisse as usize] = CAISSE;
            draw_at((x_caisse * 2) as u16, y_caisse as u16, CAISSE);
        }

        self.map[old_y as usize][old_x as usize] = VIDE;
        self.map[y as usize][x as usize] = SOK;

        self.sok_pos.x = x;
        self.sok_pos.y = y;

        draw_at((old_x * 2) as u16, old_y as u16, VIDE);
        draw_at((x * 2) as u16, y as u16, SOK);
    }
}

fn draw_at(x: u16, y: u16, c: char) {
    let mut out = stdout();
    if c == VIDE {
        execute!(out, cursor::MoveTo(x, y), Print("  ")).unwrap();
    } else {
        execute!(out, cursor::MoveTo(x, y), Print(c)).unwrap();
    }
    let _ = out.flush();
    execute!(stdout(), cursor::MoveTo(40, 40)).unwrap();
}

fn cibles(jeu: &Game) {
    for pos in &jeu.pos_cibles {
        if jeu.map[pos.y as usize][pos.x as usize] == VIDE {
            draw_at((pos.x * 2) as u16, pos.y as u16, CIBLE);
        }
    }
}

fn victory(jeu: &Game) -> bool {
    for pos in &jeu.pos_cibles {
        if jeu.map[pos.y as usize][pos.x as usize] != CAISSE {
            return false;
        }
    }

    return true;
}
