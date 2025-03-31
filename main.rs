use sdl3::event::Event;
use sdl3::pixels::Color;
use sdl3::rect::Point;

pub fn graph_cursor(initial_x: i32, initial_y: i32, cursor_color: Color) -> Result<(i32, i32), String> {
    let sdl_context = sdl3::init().map_err(|e| format!("{}", e))?;
    let video_subsystem = sdl_context.video().map_err(|e| format!("{}", e))?;

    let window = video_subsystem
        .window("Graphics Cursor", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas(); // Removed the ? here

    let mut event_pump = sdl_context.event_pump().map_err(|e| format!("{}", e))?;
    let mut mouse_x = initial_x as f32;
    let mut mouse_y = initial_y as f32;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    mouse_x = x as f32;
                    mouse_y = y as f32;
                }
                _ => {}
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Set the color for the cursor
        canvas.set_draw_color(cursor_color);

        // Draw the horizontal line of the cursor
        canvas.draw_line(
            Point::new(mouse_x as i32 - 10, mouse_y as i32),
            Point::new(mouse_x as i32 + 10, mouse_y as i32),
        ).map_err(|e| format!("{}", e))?;

        // Draw the vertical line of the cursor
        canvas.draw_line(
            Point::new(mouse_x as i32, mouse_y as i32 - 10),
            Point::new(mouse_x as i32, mouse_y as i32 + 10),
        ).map_err(|e| format!("{}", e))?;

        // Present the canvas
        canvas.present();

        // Small delay to prevent busy-looping
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok((mouse_x as i32, mouse_y as i32))
}

fn main() -> Result<(), String> {
    let initial_cursor_x = 400;
    let initial_cursor_y = 300;
    let cursor_color = Color::RGB(255, 0, 255); // Example: Magenta color

    let (final_x, final_y) = graph_cursor(initial_cursor_x, initial_cursor_y, cursor_color)?;

    println!("Graphics cursor window closed. Final mouse position: ({}, {})", final_x, final_y);

    Ok(())
}
