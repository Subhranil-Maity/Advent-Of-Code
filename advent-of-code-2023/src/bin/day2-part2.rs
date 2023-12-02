#[derive(Debug)]
struct Game {
    r: u32,
    g: u32,
    b: u32,
    id: u32,
}
impl Game {
    pub fn new(s: &str) -> Game {
        // splite the line from :
        // println!("{}", s);
        let mut game = Game {
            r: 0,
            g: 0,
            b: 0,
            id: 0,
        };
        let split_t = s.split(':').nth(0).unwrap();
        let mut f = String::new();
        for n in split_t.chars() {
            if n.is_ascii_digit() {
                f.push(n);
            }
        }
        let game_id = f.parse::<u32>().unwrap();
        game.id = game_id;
        let rounds = s.split(':').nth(1).unwrap().split(';');
        for round in rounds {
            let round = round.to_string().replace(" ", "");
            let type_t = round.split(',');
            for type_ in type_t {
                let mut f = String::new();
                for d in type_.chars() {
                    if d.is_ascii_digit() {
                        f.push(d);
                    }
                }
                let fi = f.as_str().parse::<u32>().unwrap();
                let g = type_
                    .clone()
                    .replace(f.as_str(), "")
                    .chars()
                    .next()
                    .unwrap();
                match g {
                    'r' => {
                        if game.r < fi {
                            game.r = fi
                        }
                    }
                    'g' => {
                        if game.g < fi {
                            game.g = fi
                        }
                    }
                    'b' => {
                        if game.b < fi {
                            game.b = fi
                        }
                    }
                    n => {
                        panic!("...{}...", n);
                    }
                }
            }
        }
        game
    }
}
fn main() {
    let games = include_str!("../../input/day2.txt").split('\n');
    let mut g: Vec<Game> = vec![];
    for game in games {
        g.push(Game::new(game));
    }
    let mut f = 0;
    for gs in g {
        println!("id:{} r:{} g:{} b:{} ", gs.id, gs.r, gs.g, gs.b);
        f += gs.r * gs.g * gs.b;
    }
    println!("{}", f);
}
