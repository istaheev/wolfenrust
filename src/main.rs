extern crate sdl2;

use std::f32::INFINITY;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const MAP_WIDTH: usize = 8;
const MAP_HEIGHT: usize = 8;
const TILE_HEIGHT: i32 = 64;

struct Map {
    tiles: [[i32; MAP_WIDTH]; MAP_HEIGHT],
    tile_width: u32,
    tile_height: u32,
}


#[derive(Debug)]
struct Camera {
    x: f32,
    y: f32,
    angle: f32,
}


fn draw<R>(canvas: &mut sdl2::render::Canvas<R>, map: &Map, camera: &Camera)
where
    R: sdl2::render::RenderTarget
{
    let area = canvas.viewport();
    let screen_width = area.width();
    let fov: f32 = 3.141592 / 2.0;

    // calculate distance to screen
    let screen_dist = screen_width as f32 / 2.0 / (fov/2.0).tan();

    //println!("enter draw(): camera={:?} screen_dist={}", camera, screen_dist);

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for screen_x in 0..area.width() as i32 {
        let ray_x = (screen_x - screen_width as i32 / 2) as f32;
        let ray_angle_offset = (ray_x / screen_dist).atan();
        let ray_angle = camera.angle - ray_angle_offset;
        let ray_cos = ray_angle.cos();
        let ray_sin = ray_angle.sin();

        //println!("draw() loop: screen_x={} ray_angle={} ({} degrees)", screen_x, ray_angle, ray_angle * 180.0 / 3.141);

        let tile_x0 = camera.x as i32 / map.tile_width as i32;
        let tile_y0 = camera.y as i32 / map.tile_height as i32;

        // traverse by x

        let mut dist_x = INFINITY;

        {
            let mut x;
            let tile_x_offset;

            let mut dx = map.tile_width as f32;

            if ray_cos > 0.0 {
                x = ((tile_x0 + 1) * map.tile_width as i32) as f32;
                tile_x_offset = 1;
            } else {
                dx = -dx;
                x = (tile_x0 * map.tile_width as i32) as f32;
                tile_x_offset = -1;
            }

            let dy = dx * ray_angle.tan();
            let dx0 = x - camera.x;
            let dy0 = dx0 * (dy / dx);
            let mut y = camera.y + dy0;
            let mut tile_x = tile_x0 + tile_x_offset;

            loop {
                let tile_y = y as i32 / map.tile_height as i32;
                if tile_x < 0 || tile_x >= MAP_WIDTH as i32 || tile_y < 0 || tile_y >= MAP_HEIGHT as i32 {
                    //println!("no intersection");
                    break;
                }

                if map.tiles[tile_y as usize][tile_x as usize] == 1 {
                    dist_x = ((x - camera.x) * (x - camera.x) + (y - camera.y) * (y - camera.y)).sqrt();
                    //println!("intersection x: tile_x={} tile_y={} x={} y={} dist={}", tile_x, tile_y, x, y, dist_x);
                    break;
                }

                tile_x += tile_x_offset;
                x += dx;
                y += dy;
            }
        }

        //
        // traverse by y
        //

        let mut dist_y = INFINITY;

        let mut y;
        let tile_y_offset;

        let mut dy = map.tile_height as f32;

        if ray_sin > 0.0 {
            y = ((tile_y0 + 1) * map.tile_height as i32) as f32;
            tile_y_offset = 1;
        } else {
            dy = -dy;
            y = (tile_y0 * map.tile_height as i32) as f32;
            tile_y_offset = -1;
        }

        let dx = dy / ray_angle.tan();
        let dy0 = y - camera.y;
        let dx0 = dy0 / (dy / dx);
        let mut x = camera.x + dx0;
        let mut tile_y = tile_y0 + tile_y_offset;

        loop {
            let tile_x = x as i32 / map.tile_width as i32;
            if tile_x < 0 || tile_x >= MAP_WIDTH as i32 || tile_y < 0 || tile_y >= MAP_HEIGHT as i32 {
                // println!("no intersection");
                break;
            }

            if map.tiles[tile_y as usize][tile_x as usize] == 1 {
                dist_y = ((x - camera.x) * (x - camera.x) + (y - camera.y) * (y - camera.y)).sqrt();
                // println!("intersection y: tile_x={} tile_y={} x={} y={} dist={}", tile_x, tile_y, x, y, dist_y);
                break;
            }

            tile_y += tile_y_offset;
            x += dx;
            y += dy;
        }

        let mut dist = dist_x;
        if true && dist_y < dist {
            dist = dist_y;
        }

        if dist == INFINITY {
            // Skip the line
            continue;
        }

        // line_height / TILE_HEIGHT = ray_screen_dist / dist

        let line_height = TILE_HEIGHT as f32 * screen_dist / dist  / (ray_angle - camera.angle).cos();
        let middle = area.height() as i32 / 2;

        canvas.draw_line(
            Point::new(screen_x as i32, middle - (line_height / 2.0) as i32),
            Point::new(screen_x as i32, middle + (line_height / 2.0) as i32),
        ).unwrap();
    }
}


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "Wolfenrust 3D",
            800,
            600
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL renderer: {:?}", canvas.info());

    let tiles = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let map = Map {
        tiles,
        tile_width: 64,
        tile_height: 64,
    };

    let mut camera = Camera {
        x: (2 * map.tile_width + 5) as f32,
        y: (2 * map.tile_height + 5) as f32,
        angle: 0.0,
    };

    let mut frames = 0;
    let mut start_time = std::time::Instant::now();

    let mut event_pump = sdl_context.event_pump()?;
    'mainloop: loop {
        for ev in event_pump.poll_iter() {
            match ev {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    const SPEED: f32 = 1.0;
                    camera.x += SPEED * camera.angle.cos();
                    camera.y += SPEED * camera.angle.sin();
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    const SPEED: f32 = 1.0;
                    camera.x -= SPEED * camera.angle.cos();
                    camera.y -= SPEED * camera.angle.sin();
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    const SPEED: f32 = 0.05;
                    camera.angle += SPEED;
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    const SPEED: f32 = 0.05;
                    camera.angle -= SPEED;
                },
                _ => {},
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        draw(&mut canvas, &map, &camera);

        canvas.present();

        frames += 1;

        let elapsed = start_time.elapsed();
        if elapsed.as_secs() > 2 {
            println!("FPS: {:.1}", frames as f32/ elapsed.as_secs() as f32);
            frames = 0;
            start_time = std::time::Instant::now();
        }
    }

    Ok(())
}