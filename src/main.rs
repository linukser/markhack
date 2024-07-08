extern crate ncurses;
extern crate rand;

use ncurses::*;
use rand::Rng;
use std::cmp;

const PLAYER: char = '@';
const MONSTER: char = 'M';
const FLOOR: char = '.';
const WALL: char = '#';

struct Game {
    player_x: i32,
    player_y: i32,
    map: Vec<Vec<char>>,
    monsters: Vec<(i32, i32)>,
    width: i32,
    height: i32,
}

impl Game {
    fn new(width: i32, height: i32) -> Self {
        let mut game = Self {
            player_x: 1,
            player_y: 1,
            map: vec![vec![WALL; width as usize]; height as usize],
            monsters: vec![],
            width,
            height,
        };
        game.generate_map();
        game.spawn_monsters(5);
        game
    }

    fn generate_map(&mut self) {
        let mut rng = rand::thread_rng();
        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                if rng.gen_range(0..10) > 1 {
                    self.map[y as usize][x as usize] = FLOOR;
                }
            }
        }
        self.map[self.player_y as usize][self.player_x as usize] = FLOOR;
    }

    fn spawn_monsters(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let mut x;
            let mut y;
            loop {
                x = rng.gen_range(1..self.width-1);
                y = rng.gen_range(1..self.height-1);
                if self.map[y as usize][x as usize] == FLOOR {
                    break;
                }
            }
            self.monsters.push((x, y));
        }
    }

    fn draw(&self) {
        clear();
        for (y, row) in self.map.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                mvaddch(y as i32, x as i32, tile as u32);
            }
        }
        attron(COLOR_PAIR(2));
        mvaddch(self.player_y, self.player_x, PLAYER as u32);
        attroff(COLOR_PAIR(2));
        for &(mx, my) in &self.monsters {
            attron(COLOR_PAIR(1));
            mvaddch(my, mx, MONSTER as u32);
            attroff(COLOR_PAIR(1));
        }
        refresh();
    }

    fn update(&mut self, ch: i32) {
        let (dx, dy) = match ch {
            KEY_UP => (0, -1),
            KEY_DOWN => (0, 1),
            KEY_LEFT => (-1, 0),
            KEY_RIGHT => (1, 0),
            _ => (0, 0),
        };

        let new_x = cmp::max(1, cmp::min(self.width - 2, self.player_x + dx));
        let new_y = cmp::max(1, cmp::min(self.height - 2, self.player_y + dy));

        if self.map[new_y as usize][new_x as usize] == FLOOR {
            self.player_x = new_x;
            self.player_y = new_y;
        }

        for (i, &(mx, my)) in self.monsters.iter().enumerate() {
            if mx == self.player_x && my == self.player_y {
                self.monsters.remove(i);
                break;
            }
        }
    }
}

fn main() {
    // Initialize ncurses
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Initialize color pairs
    if has_colors() {
        start_color();
        init_pair(1, COLOR_RED, COLOR_BLACK);
        init_pair(2, COLOR_GREEN, COLOR_BLACK);
        init_pair(3, COLOR_WHITE, COLOR_BLACK);
    }

    // Set color for map
    attron(COLOR_PAIR(3));

    let mut game = Game::new(80, 24);

    loop {
        game.draw();
        let ch = getch();
        if ch == 'q' as i32 {
            break;
        }
        game.update(ch);
    }

    // End ncurses
    endwin();
}


