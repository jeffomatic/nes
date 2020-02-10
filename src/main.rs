extern crate regex;
extern crate sdl2;

#[macro_use]
extern crate lazy_static;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;
use std::time::Duration;

mod cpu;
mod mapper;
mod math;
mod ppu;

pub fn main() {
    let (_, mapper_ppu) = mapper::test::new();
    let mut ppu = ppu::Ppu::new(Box::new(mapper_ppu));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let scale = 3;

    let window = video_subsystem
        .window(
            "nes",
            scale * ppu::SCREEN_WIDTH as u32,
            scale * ppu::SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    canvas.clear();
    canvas.present();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture(
            PixelFormatEnum::RGB24,
            TextureAccess::Streaming,
            ppu::SCREEN_WIDTH as u32,
            ppu::SCREEN_HEIGHT as u32,
        )
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return,
                _ => {}
            }
        }

        i = (i + 1) % 255;
        let r: u8 = i;
        let g: u8 = 64;
        let b: u8 = 255 - i;

        for i in 0..ppu::SCREEN_HEIGHT {
            for j in 0..ppu::SCREEN_WIDTH {
                let base = i * ppu::SCREEN_ROW_PITCH + (3 * j);
                ppu.framebuf[base + 0] = r;
                ppu.framebuf[base + 1] = g;
                ppu.framebuf[base + 2] = b;
            }
        }

        texture.update(None, &ppu.framebuf, ppu::SCREEN_ROW_PITCH);
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
