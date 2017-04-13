extern crate piston_window;
extern crate find_folder;

mod apples;
use apples::Apples;

use piston_window::*;

// get_current_pos(playfield) -> coord
// update_d(d, coord, playfield) -> playfield

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("draw an image", [400, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut apples = Apples::new();

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

    let mut up_d = false;
    let mut down_d = false;
    let mut right_d = false;
    let mut left_d = false;

    let drawfactor = 90.0;

    let mut playfield = vec![
        vec![0.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0]];

    window.set_lazy(true); // ??

    // iterate over window events
    while let Some(e) = window.next() {

        if let Some(upd) = e.render_args() {

            if up_d {
                apples.is_gone(1);
            }

            /*
            // @todo change x and y names, it's confusing
            for (xi, xv) in  playfield.iter().enumerate() {
                for (yi,  yv) in xv.iter().enumerate() {

                    // ideally we'd call update_d in every branch
                    // instead of assigning playfield here

                    println!("x {:?} y {:?} value {:?}", yi, xi, yv);
                    if up_d {
                        playfield[yi as usize][xi as usize] = 0.0;
                        playfield[yi as usize -1][xi as usize] = 1.0; // @todo refactor to use a funciton to return next elem
                    }
                    if down_d {
                        playfield[yi as usize][xi as usize] = 0.0;
                        playfield[yi + 1][xi as usize] = 1.0;
                    }
                    if right_d {
                        playfield[yi as usize][xi as usize] = 0.0;
                        playfield[yi as usize][xi + 1] = 1.0;
                    }
                    if left_d {
                        playfield[yi as usize][xi as usize] = 0.0;
                        playfield[yi as usize][xi -1] = 1.0;
                    }
                }
            }*/
        }

        if let Some(button) = e.press_args() {
            println!("Pressed {:?}", button);
            match button {
                Button::Keyboard(Key::W) => {
                    up_d = true;
                }
                Button::Keyboard(Key::S) => {
                    down_d = true;
                }
                Button::Keyboard(Key::A) => {
                    left_d = true;
                }
                Button::Keyboard(Key::D) => {
                    right_d = true;
                }
                _ => {}
            }
        }

        if let Some(button) = e.release_args() {
            // println!("Released {:?}", button);
            match button {
                Button::Keyboard(Key::W) => {
                    up_d = false;
                    //update_d("up");
                }
                Button::Keyboard(Key::S) => {
                    down_d = false;
                    //update_d("down");
                }
                Button::Keyboard(Key::A) => {
                    left_d = false;
                    //update_d("left");
                }
                Button::Keyboard(Key::D) => {
                    right_d = false;
                    //update_d("right");
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

            for (xi, xv) in playfield.iter().enumerate() {
                for yi in xv {
                    // println!("x {:?} y {:?} value {:?}", yi, xi, yv);
                    if yi == &1.0 {
                        let square = rectangle::square(0.0, 0.0, 50.0);
                        rectangle(color, square, c.transform.trans(
                            (drawfactor + (drawfactor * yi as &f64)),
                            (drawfactor + (drawfactor * xi as f64))), g);
                    }
                }
            }

        });

    }
}
