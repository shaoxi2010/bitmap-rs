use minifb::{Key, Window, WindowOptions};
use bitmap::{ARGB32, DrawIo, Minifb, WHITE};

const WIDTH: usize = 800;
const HEIGHT: usize = 480;

const PROGRESS_WIDTH:usize = 600;
const PROGRESS_HEIGHT:usize = 200;

fn main() {
    let mut buffer = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut x = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        ARGB32::painter(&mut buffer, WIDTH, HEIGHT, |bitmap| {
            bitmap.draw_text((100,100).into(), WHITE, &format!("Count:{}", x), 32)?;
            bitmap.draw_rectagle(((WIDTH - PROGRESS_WIDTH) / 2, (HEIGHT - PROGRESS_HEIGHT) / 2).into(), PROGRESS_WIDTH, PROGRESS_HEIGHT, WHITE)?; 
            Ok(())
        }).unwrap();
        x += 1;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}