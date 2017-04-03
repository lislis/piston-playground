extern crate piston_window;
extern crate find_folder;

use piston_window::*;

/*
struct Game {
    x: f64,
    y: f64
}

impl Game {
    fn new() -> Game {
        Game { x: 0.0, y: 0.0 }
    }
    fn on_update(&mut self, upd: UpdateArgs) {
        self.x += 5.0 * upd.dt;
        self.y += 5.0 * upd.dt;
    }
}
*/

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("draw an image", [400, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3,3)
        .for_folder("assets").unwrap();
    let house = assets.join("house.jpg");
    let house = Texture::from_path(
        &mut window.factory,
        &house,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let ref font = assets.join("Amatic-Bold.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    let color = [0.3, 0.5, 0.5, 1.0];

//    let mut game = Game::new();

    window.set_lazy(true);
    // game loop
    // iterate over window events
    while let Some(e) = window.next() {

        if let Some(button) = e.press_args() {
            if button == Button::Keyboard(Key::W) {
                println!("Pressed {:?}", button);
            }
        }

        window.draw_2d(&e, |c, g| {
            let font_transform = c.transform.trans(10.0, 100.0);

            clear([1.0; 4], g);
            image(&house, c.transform.trans(50.0, 0.0), g);
            text::Text::new_color(color, 42).draw(
                "POINTS",
                &mut glyphs,
                &c.draw_state,
                font_transform,
                g
            );

            let square = rectangle::square(0.0, 0.0, 30.0);
            rectangle(color, square, c.transform.trans(200.0, 300.0), g);
        });

    }
}
