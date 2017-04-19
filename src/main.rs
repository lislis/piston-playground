extern crate piston_window;
extern crate find_folder;

mod apples;
mod throwable;
mod player;

use player::Player;

//use piston_window::{Button, EventLoop, Input, Motion, OpenGL, PistonWindow, WindowSettings};
use piston_window::*;

struct Game {
    pub folks: Vec<Folk>
}

impl Game {
    pub fn new() -> Game {
        Game {
            folks: vec![]
        }
    }
    pub fn new_folk (&mut self) {
        self.folks.push(Folk::new());
    }

}


struct Folk {
    pub x: f64,
    pub y: f64,
    moving: bool,
    ltr: bool,
    speed: f64
}

impl Folk {
    pub fn new() -> Folk {
        Folk {
            x: 0.0,
            y: 500.0,
            moving: true,
            ltr: true,
            speed: 1.0
        }
    }
    pub fn update(&mut self) {
        if self.moving {
            if self.ltr {
                self.x += 1.0;
            } else {
                self.x -= 1.0;
            }
        }
    }
}

fn calc_dis_pos(pos: f64, factor: f64) -> f64 {
    factor + (pos * factor)
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("draw an image", [400, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .samples(8)
        .build()
        .unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut player = Player::new();

    let COLOR = [0.3, 0.5, 0.5, 1.0];

    let assets = find_folder::Search::ParentsThenKids(3,3)
        .for_folder("assets").unwrap();
    let house = assets.join("house.jpg");
    let house = Texture::from_path(
        &mut window.factory,
        &house,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let apple = Texture::from_path(
        &mut window.factory,
        &assets.join(player.apples.full),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let apple_gone = Texture::from_path(
        &mut window.factory,
        &(assets.join(player.apples.empty)),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let ref font = assets.join("Amatic-Bold.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    let score = "Points: ";

    let drawfactor = 90.0;

    let mut game = Game::new();
    game.new_folk();

    let mut count = 0.0;

    window.set_lazy(true); // ??

    // iterate over window events
    while let Some(e) = window.next() {
        match e {
            Input::Release(Button::Keyboard(key)) => {
                match key {
                    Key::W => {
                        player.moving(0, -1)
                    }
                    Key::S => {
                        player.moving(0, 1);
                    }
                    Key::A => {
                        player.moving(-1,0);
                    }
                    Key::D => {
                        player.moving(1, 0);
                    }
                    Key::K => {
                        let x = calc_dis_pos(player.x as f64, drawfactor);
                        let y = calc_dis_pos(player.y as f64, drawfactor);
                        //throwable.set_active(x, y);
                        player.throw(x, y);
                    }
                    _ => {}
                }
            }

            Input::Update(args) => {
                count += args.dt;
                println!("update {}", count);
            }

            Input::Render(_) => {
                println!("render {}", count);
                window.draw_2d(&e, |c, g| {
                    let font_transform = c.transform.trans(10.0, 100.0);

                    clear([1.0; 4], g);
                    image(&house, c.transform.trans(50.0, 0.0), g);
                    text::Text::new_color(COLOR, 42).draw(
                        score,
                        &mut glyphs,
                        &c.draw_state,
                        font_transform,
                        g
                    );

                    for (i, v) in (0..player.apples.total).enumerate() {
                        if i as i32 >= player.apples.left {
                            image(&apple_gone, c.transform.scale(0.5, 0.5).trans((0.0 + (i * 50) as f64), 0.0), g);
                        } else {
                            image(&apple, c.transform.scale(0.5, 0.5).trans((0.0 + (i * 50) as f64), 0.0), g);
                        }
                    }

                    for (ci, cv) in (0..player.rows).enumerate() {
                        for (ri, rv) in (0..player.columns).enumerate() {
                            if rv == player.x && cv == player.y {
                                let square = rectangle::square(0.0, 0.0, 50.0);
                                rectangle(COLOR, square, c.transform.trans(
                                    calc_dis_pos(rv as f64, drawfactor),
                                    calc_dis_pos(cv as f64, drawfactor)), g);
                                image(&apple, c.transform.trans(
                                    calc_dis_pos(rv as f64, drawfactor),
                                    calc_dis_pos(cv as f64, drawfactor)).scale(0.9, 0.9), g);
                            }

                        }
                    }

                    for t in player.throwable.iter() {
                        image(&apple, c.transform.trans(
                            t.x, t.y), g);
                    }

                    for f in game.folks.iter() {
                        let folk_square = rectangle::square(0.0,0.0, 40.0);
                        rectangle(COLOR, folk_square, c.transform.trans(
                            f.x, f.y), g);
                    }
                });

            }
            _ => {}
        }

    }
}
