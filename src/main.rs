extern crate piston_window;
extern crate find_folder;

mod apples;
use apples::Apples;

mod player;
use player::Player;

use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("draw an image", [400, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut apples = Apples::new();
    let mut player = Player::new();

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
        &assets.join(apples.full),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let apple_gone = Texture::from_path(
        &mut window.factory,
        &(assets.join(apples.empty)),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let ref font = assets.join("Amatic-Bold.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    let color = [0.3, 0.5, 0.5, 1.0];

    let score = "Points: ";

    let drawfactor = 90.0;

    window.set_lazy(true); // ??

    // iterate over window events
    while let Some(e) = window.next() {

        if let Some(upd) = e.render_args() {

        }

        if let Some(button) = e.release_args() {
            // println!("Released {:?}", button);
            match button {
                Button::Keyboard(Key::W) => {
                    player.moving(0, -1)
                }
                Button::Keyboard(Key::S) => {
                    player.moving(0, 1);
                }
                Button::Keyboard(Key::A) => {
                    player.moving(-1,0);
                }
                Button::Keyboard(Key::D) => {
                    player.moving(1, 0);
                }
                _ => {}
            }
        }

        window.draw_2d(&e, |c, g| {
            let font_transform = c.transform.trans(10.0, 100.0);

            clear([1.0; 4], g);
            image(&house, c.transform.trans(50.0, 0.0), g);
            text::Text::new_color(color, 42).draw(
                score,
                &mut glyphs,
                &c.draw_state,
                font_transform,
                g
            );

            for (i, v) in (0..apples.total).enumerate() {
                if i as i32 >= apples.left {
                    image(&apple_gone, c.transform.scale(0.5, 0.5).trans((0.0 + (i * 50) as f64), 0.0), g);
                } else {
                    image(&apple, c.transform.scale(0.5, 0.5).trans((0.0 + (i * 50) as f64), 0.0), g);
                }
            }

            for (ci, cv) in (0..player.rows).enumerate() {
                for (ri, rv) in (0..player.columns).enumerate() {
                    if rv == player.x && cv == player.y {
                        let square = rectangle::square(0.0, 0.0, 50.0);
                        rectangle(color, square, c.transform.trans(
                            (drawfactor + (drawfactor * rv as  f64)),
                            (drawfactor + (drawfactor * cv as f64))), g);
                    }
                }
            }
        });

    }
}
