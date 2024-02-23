
//! A Roguelike Game using Piston Engine

extern crate glutin_window;
extern crate piston;

extern crate graphics;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use piston::WindowSettings;

use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::RenderEvent;
use piston::input::{Button,ButtonState,Key};
use piston ::ButtonEvent;
use graphics::Character::CharacterCache;

use opengl_graphics::{GlGraphics, Filter, OpenGL,GlyphCache, TextureSettings};
type Color= [f32;4];
 const RED: Color = [1.0, 0.0, 0.0, 1.0];
    const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
    const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
    const WHITE: Color = [1.0; 4];
	//const BLACK: Color={0.0;4};
	// Game Properties
	const WINDOW_SIZE: i32=512;
	const PIXEL_SIZE: f64=32.0;
	const WORLD_SIZE: i32= WINDOW_SIZE/PIXEL_SIZE as i32;
	 type Map =Vec<Vec<Tile>>;
	// defining what a tile is 
#[derive(Clone)]
	struct Tile 
	{
		color:Color,
	}

	#[derive(Clone)]
	// object struct declaration
	struct Object
	{
		xpos :: i32,
		ypos :: i32,
		character:char,
		color:Color,
	}
	impl Object
	{
		pub fn new ()
	}
	// implement the struct
	impl Tile
	
	{
		pub fn empty() -> Self
		{
			Tile
			{
				color:RED
			}
		}
		
	
		pub fn wall() -> Self
		{
			Tile
			{
				color:GREEN
			}
		}
}
pub fn MakeMap() ->Map
	{
	let mut map = vec![vec![Tile::empty();WORLD_SIZE as usize]; WORLD_SIZE as usize];
		map[WORLD_SIZE as usize/2][WORLD_SIZE as usize/2]=Tile::wall();
		
        map
	}
fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Roguelike", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    let map = MakeMap();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |_c, g| {
                graphics::clear(BLUE, g);
				for i in 0 .. WORLD_SIZE
	{
		for j in 0 .. WORLD_SIZE
		{
			let pos: [f64;4]=[
				PIXEL_SIZE*i as f64,
				PIXEL_SIZE *j as f64,
				PIXEL_SIZE *(i+1) as f64,
				PIXEL_SIZE*(j+1) as f64,
			];                                         
			// draw tile
			graphics::Rectangle::new(map[i as usize][j as usize].color).draw
			(
				pos,
				&_c.draw_state,
				_c.transform,
				g,
			
			);
		}
		
	}
   })
	}
  }
}
