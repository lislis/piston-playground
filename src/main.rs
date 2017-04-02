extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("draw an image", [300, 600])
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
    window.set_lazy(true);

    // game loop
    // iterate over window events
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            image(&house, c.transform, g);
        });
    }
}
