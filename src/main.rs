use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    style::Print,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};
use std::time::Duration;

type tab2D = Vec<Vec<char>>;

const SOK: char = '💂';
const CAISSE: char = '📦';
const MUR: char = '⬛';
const CIBLE: char = '🔸';
const VIDE: char = ' ';

const UP: dep_soko = dep_soko { x: 0, y: -1 };
const DOWN: dep_soko = dep_soko { x: 0, y: 1 };
const RIGHT: dep_soko = dep_soko { x: 1, y: 0 };
const LEFT: dep_soko = dep_soko { x: -1, y: 0 };

const UP_KEY: char = 'z';
const DOWN_KEY: char = 's';
const LEFT_KEY: char = 'q';
const RIGHT_KEY: char = 'd';
const UNDO_KEY: char = 'u';
const LEAVE_KEY: char = 'x';

struct dep_soko {
    x: i32,
    y: i32,
}

struct Pos {
    x: i32,
    y: i32,
}

struct Game {
    map: tab2D,
    sok_pos: Pos,
    pos_cibles: Vec<Pos>,
    tab_dep: Vec<char>,
}

fn main() {
    // rawmode & alternate screen
    let mut out = stdout();
    execute!(out, EnterAlternateScreen).unwrap();
    let _ = enable_raw_mode();

    // initialisation
    let mut jeu = Game::init();
    execute!(out, cursor::MoveTo(0, 0)).unwrap();

    jeu.show();

    loop {
        if let Ok(k) = key_pressed() {
            match k {
                UP_KEY => jeu.MoveSoko(UP, UP_KEY),
                DOWN_KEY => jeu.MoveSoko(DOWN, DOWN_KEY),
                LEFT_KEY => jeu.MoveSoko(LEFT, LEFT_KEY),
                RIGHT_KEY => jeu.MoveSoko(RIGHT, RIGHT_KEY),
                UNDO_KEY => jeu.undo(),
                LEAVE_KEY => break,
                _ => (),
            }

            jeu.cibles();

            if jeu.victory() {
                break;
            }
        }
    }

    // game end
    let _ = disable_raw_mode();
    if jeu.victory() {
        println!("\nVICTOIRE !");
        for dep in &jeu.tab_dep {
            print!("{} ", dep);
        }
        println!();
    }
}

// Game methods
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
            tab_dep: vec![],
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

    fn victory(&self) -> bool {
        for pos in &self.pos_cibles {
            if self.map[pos.y as usize][pos.x as usize] != CAISSE {
                return false;
            }
        }

        return true;
    }

    fn cibles(&self) {
        for pos in &self.pos_cibles {
            if self.map[pos.y as usize][pos.x as usize] == VIDE {
                draw_at((pos.x * 2) as u16, pos.y as u16, CIBLE);
            }
        }
    }

    fn undo(&mut self) {
        /*let dep: char = self.tab_dep.pop().unwrap();
        match dep {
            UP_KEY => {draw_at((self.sok_pos.x * 2) as u16, self.sok_pos.y as u16, CAISSE);
                    draw_at((self.sok_pos.x + DOWN.x * 2) as u16, (self.sok_pos.y + DOWN.y) as u16, SOK);},
            DOWN_KEY => ,
            RIGHT_KEY => ,
            LEFT_KEY => ,
            UP_KEY => ,
            DOWN_KEY => ,
            RIGHT_KEY => ,
            LEFT_KEY => ,
            _ => (),
        }*/
    }

    fn MoveSoko(&mut self, dep_sok: dep_soko, key: char) {
        let old_x = self.sok_pos.x;
        let old_y = self.sok_pos.y;

        let x = old_x + dep_sok.x;
        let y = old_y + dep_sok.y;

        let cible = self.map[y as usize][x as usize];

        let mut dep: char = key;

        // checking
        if x < 0 || x >= 15 || y < 0 || y >= 15 {
            return;
        }

        if cible == MUR {
            return;
        }

        if cible == CAISSE {
            let x_caisse = x + dep_sok.x;
            let y_caisse = y + dep_sok.y;

            if x_caisse < 0 || x_caisse >= 15 || y_caisse < 0 || y_caisse >= 15 {
                return;
            }

            let next_cible = self.map[y_caisse as usize][x_caisse as usize];
            if next_cible == MUR || next_cible == CAISSE {
                return;
            }

            dep = key.to_ascii_uppercase();
            self.map[y_caisse as usize][x_caisse as usize] = CAISSE;
            draw_at((x_caisse * 2) as u16, y_caisse as u16, CAISSE);
        }

        self.map[old_y as usize][old_x as usize] = VIDE;
        self.map[y as usize][x as usize] = SOK;

        self.sok_pos.x = x;
        self.sok_pos.y = y;

        draw_at((old_x * 2) as u16, old_y as u16, VIDE);
        draw_at((x * 2) as u16, y as u16, SOK);

        self.tab_dep.push(dep);
    }
}

// functions
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

fn key_pressed() -> Result<char, bool> {
    if event::poll(Duration::from_millis(10)).unwrap_or(false) {
        if let Ok(Event::Key(key_pressed)) = event::read() {
            if key_pressed.kind == KeyEventKind::Press {
                return match key_pressed.code {
                    KeyCode::Char(UP_KEY) => Ok(UP_KEY),
                    KeyCode::Char(DOWN_KEY) => Ok(DOWN_KEY),
                    KeyCode::Char(RIGHT_KEY) => Ok(RIGHT_KEY),
                    KeyCode::Char(LEFT_KEY) => Ok(LEFT_KEY),
                    KeyCode::Char(LEAVE_KEY) => Ok(LEAVE_KEY),
                    KeyCode::Char(UNDO_KEY) => Ok(UNDO_KEY),
                    _ => Err(false),
                };
            }
        }
    }
    Err(false)
}
