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

struct DepSoko {
    x: i32,
    y: i32,
}

struct Direction {
    dep: DepSoko,
    key: char,
    caisse: char,
}

const UP: Direction = Direction {
    dep: DepSoko { x: 0, y: -1 },
    key: 'z',
    caisse: 'Z',
};

const DOWN: Direction = Direction {
    dep: DepSoko { x: 0, y: 1 },
    key: 's',
    caisse: 'S',
};

const LEFT: Direction = Direction {
    dep: DepSoko { x: -1, y: 0 },
    key: 'q',
    caisse: 'Q',
};

const RIGHT: Direction = Direction {
    dep: DepSoko { x: 1, y: 0 },
    key: 'd',
    caisse: 'D',
};

const UNDO_KEY: char = 'u';
const LEAVE_KEY: char = 'x';

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
                x if x == UP.key => jeu.MoveSoko(UP.dep, UP.key),
                x if x == DOWN.key => jeu.MoveSoko(DOWN.dep, DOWN.key),
                x if x == LEFT.key => jeu.MoveSoko(LEFT.dep, LEFT.key),
                x if x == RIGHT.key => jeu.MoveSoko(RIGHT.dep, RIGHT.key),
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

    fn update_tile(&mut self, x: i32, y: i32, tile: char) {
        self.map[y as usize][x as usize] = tile;
        draw_at((x * 2) as u16, y as u16, tile);
    }

    fn undo(&mut self) {
        let key: char = self.tab_dep.pop().unwrap();
        let (dep, direct) = dep_inverse(key).unwrap();

        if box_moved(key) {
            self.update_tile(self.sok_pos.x, self.sok_pos.y, CAISSE);
            self.update_tile(self.sok_pos.x, self.sok_pos.y, VIDE);
        } else {
            self.update_tile(
                (self.sok_pos.x + direct.dep.x),
                (self.sok_pos.y + direct.dep.y),
                VIDE,
            );
        }

        self.update_tile((self.sok_pos.x + dep.x), (self.sok_pos.y + dep.y), SOK);
    }

    fn MoveSoko(&mut self, dep_sok: DepSoko, key: char) {
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

            // uppercase == caisse bouger
            dep = key.to_ascii_uppercase();

            // map + stdout update
            self.update_tile(x_caisse, y_caisse, CAISSE);
        }

        self.sok_pos.x = x;
        self.sok_pos.y = y;

        self.update_tile(old_x, old_y, VIDE);
        self.update_tile(x, y, SOK);

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
                    x if x == KeyCode::Char(UP.key) => Ok(UP.key),
                    x if x == KeyCode::Char(DOWN.key) => Ok(DOWN.key),
                    x if x == KeyCode::Char(RIGHT.key) => Ok(RIGHT.key),
                    x if x == KeyCode::Char(LEFT.key) => Ok(LEFT.key),
                    KeyCode::Char(LEAVE_KEY) => Ok(LEAVE_KEY),
                    KeyCode::Char(UNDO_KEY) => Ok(UNDO_KEY),
                    _ => Err(false),
                };
            }
        }
    }
    Err(false)
}
fn dep_inverse(c: char) -> Result<(DepSoko, Direction), bool> {
    match c.to_ascii_lowercase() {
        x if x == UP.key => Ok((DOWN.dep, UP)),
        x if x == DOWN.key => Ok((UP.dep, DOWN)),
        x if x == LEFT.key => Ok((RIGHT.dep, LEFT)),
        x if x == RIGHT.key => Ok((LEFT.dep, RIGHT)),
        _ => Err(false),
    }
}
fn box_moved(c: char) -> bool {
    return c == UP.caisse || c == DOWN.caisse || c == LEFT.caisse || c == RIGHT.caisse;
}
