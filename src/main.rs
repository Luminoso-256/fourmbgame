/**
4MB Game
~ An attempt to make a game with a final size of 4 megabytes, or less. ~
idea: simple sidescroller sandbox.
**/
use macroquad::prelude::*;
use std::string;
use crate::constants::*;

mod constants;

///Spacing constant for primary tile renderer
const TILE_SIZE: f32 = 8.0;

fn get_linear_noise(pos:f32) -> f32{
    let mut x_val = f64::sin((2f64 * (pos) as f64) as f64) + f64::sin(std::f64::consts::PI*((pos) as f64));
    x_val as f32
}

fn normalize(min:f32, max:f32, num:f32) -> f32{
    let delta = max-min;
    return (num - min) / delta;
}

struct Tile {
    x: i32,
    y: i32,
    t: i32,
}

struct Entity{
    pos:Vec2,
    t: i32,
    data: String
}

struct World {
    tiles: Vec<Tile>,
    entities: Vec<Entity>,
    player:Entity,
    camera:Camera2D,
    time: i32,
}

impl World{
    fn is_tile_at(&self, x:i32, y:i32, ) -> bool{
        for tile in &self.tiles{
            if tile.x == x && tile.y == y{
                return true;
            }
        }
        false
    }
    fn get_tile_at(&self, x:i32, y:i32) -> Tile {
        for tile in &self.tiles{
            if tile.x == x && tile.y == y{
                return Tile{
                    x,
                    y,
                    t: tile.t
                }
            }
        }
        Tile{
            x,
            y,
            t: 0
        }
    }
}




#[macroquad::main("<4MB Game Time!")]
async fn main() {
    //=== File Includes
    let spr_slot = include_str!("asset/slot.spr.txt");
    let spr_grass = include_str!("asset/grass.spr.txt");
    let spr_dirt = include_str!("asset/dirt.spr.txt");
    let spr_water = include_str!("asset/water.spr.txt");
    let spr_log = include_str!("asset/log.spr.txt");
    let spr_leaf = include_str!("asset/leaf.spr.txt");
    let spr_player = include_str!("asset/player.spr.txt");
    //=== camera & world setup
    let mut cam_zoom = vec2(0.0019,-(screen_width() / screen_height())*0.002665);
    let mut level = World {
        tiles: vec![],
        entities: vec![],
        player: Entity {
            pos: vec2(10.0, 10.0),
            t: 0,
            data: "".to_string()
        },
        camera: Camera2D{
            zoom: vec2(0.0019, -(screen_width() / screen_height())*0.002665),
            target : Vec2::new(2672., 264.),
            rotation : 0.,
            ..Default::default()
        },
        time: 0,
    };

    level.camera.zoom.y = -level.camera.zoom.y;

    //=== Worldgen
    let mut noise_offset = rand::gen_range(0.0, 20.0);
    for x in 0..100{
        let mut noise = get_linear_noise(noise_offset);
        noise = noise*1.2;
        noise = noise+11.;

        println!("Noise Val: {}", noise);
        noise_offset += 0.1;


        //Surface Tile
        level.tiles.push(Tile {
            x,
            y: noise as i32,
            t: 0,
        });
        //Base underground
        for i in (noise as i32)+1.. 20{
            level.tiles.push(Tile {
                x,
                y:i,
                t:1,
            });
        }
        //Features: Trees
        if rand::gen_range(0, 10) > 8{
            let tree_trunk_height = rand::gen_range(3,5);
            for i in noise as i32  - tree_trunk_height.. noise as i32 {
                level.tiles.push(Tile {
                    x,
                    y:i,
                    t:2,
                });
            }
            //hardcoded canopy
            level.tiles.push(Tile {
                x: x+1,
                y:noise as i32  - tree_trunk_height,
                t:3,
            });
            level.tiles.push(Tile {
                x: x-1,
                y:noise as i32  - tree_trunk_height,
                t:3,
            });
            level.tiles.push(Tile {
                x: x,
                y:noise as i32  - tree_trunk_height -1,
                t:3,
            });
        }
        //Features: Ponds
        //TODO:Ponds

    }




    loop {

        //==== Process Input

        if is_key_down(KeyCode::W){
            level.player.pos.y -= 0.1;
        }
        if is_key_down(KeyCode::A){
            level.player.pos.x -= 0.1;
        }
        if is_key_down(KeyCode::S){
            level.player.pos.y += 0.1;
        }
        if is_key_down(KeyCode::D){
            level.player.pos.x += 0.1;
        }

        //==== Game logic & other chores


        //==== Handle Rendering
        clear_background(WHITE);
        cam_zoom.y = -(screen_width() / screen_height())*0.001865;
        set_camera(&level.camera);
        level.camera.zoom = cam_zoom;
        /* Terrain */
        for tile in &level.tiles {
            match tile.t {
                -1 => {}
                0 => draw_spr(spr_grass, tile.x*TILE_SIZE as i32, tile.y*TILE_SIZE as i32, 4.0), //GRASS
                1 => draw_spr(spr_dirt, tile.x*TILE_SIZE as i32, tile.y*TILE_SIZE as i32, 4.0), //DIRT
                2 => draw_spr(spr_log, tile.x*TILE_SIZE as i32, tile.y*TILE_SIZE as i32, 4.0), //LOG
                3 => draw_spr(spr_leaf, tile.x*TILE_SIZE as i32, tile.y*TILE_SIZE as i32, 4.0), //LEAF
                4 => draw_spr(spr_water, tile.x*TILE_SIZE as i32, tile.y*TILE_SIZE as i32, 4.0), //WATER
                //if we screw up, log it.
                _ => println!("[Render] [Error] Encountered tile ID {}, which has no corresponding branch in the match statement.", tile.t),
            }
            draw_text(&*format!("{}/{}/{}", tile.x, tile.y, tile.t), tile.x as f32 *TILE_SIZE*1.5, tile.y as f32*TILE_SIZE*1.5, 20.0, BLACK);
        }
        /* Entities */
        //player
        draw_spr(spr_water, level.player.pos.x as i32, level.player.pos.x as i32, 10.0);
        //world entities
        for entity in &level.entities{
            match entity.t{
                //0 -> Player
                //1 -> Blob
                //2 -> Zombie
                _ => println!("[Render] [Error] Encountered entity render ID {}, which has no corresponding branch in the match statement", entity.t)
            }
        }
        set_default_camera();
        draw_text(&*format!("Px/Py: {} | FPS: {}", &level.player.pos, get_fps()), 10.0, 10.0, 20.0, BLACK);
        //some test mouse code!

      //  if level.is_tile_at(mouse_position().0 as i32, mouse_position().1 as i32){
      //      draw_text(&*format!("hovering over tile of type {}", level.get_tile_at(mouse_position().0 as i32, mouse_position().1 as i32).t), mouse_position().0, mouse_position().1, 10.0, BLACK);
      //  }
      //  else{
      //      draw_text(&*format!("no tile hovered. {:?}", mouse_position()), mouse_position().0, mouse_position().1, 10.0, BLACK);
      //  }

        /* UI & Debug */

        next_frame().await
    }
}

///draws an image in the .spr.txt format to a specified set of cordinates, with a given pixel unit size
fn draw_spr(spr_file: &str, draw_x: i32, draw_y: i32, pixel_size: f32) {
    let mut curr_x = 0;
    let mut curr_y = 0;

    let lines = spr_file.split("|");
    for line in lines {
        let pixels = line.split(" ");
        for pixel in pixels {
            let mut color: Color = Default::default();
            match pixel {
                "0" => color = COL_0,
                "1" => color = BLACK,
                "2" => color = COL_2,
                "3" => color = COL_3,
                "4" => color = COL_4,
                "5" => color = COL_5,
                "6" => color = COL_6,
                "7" => color = COL_7,
                "8" => color = COL_8,
                "A" => color = WHITE,
                "B" => color = COL_B,
                "C" => color = COL_C,
                _ => println!("[Render] [spr.txt renderer] [Error] Incorrect color code has been used: {} at pos {},{}", pixel, curr_x, curr_y)
            }

            // draw_x/y are the cordinates of the image on the screen
            // curr x/y are the cordinates of this pixel, in that image, as we're iterating over the data.
            let base_x = curr_x + draw_x;
            let base_y = curr_y + draw_y;
            //draw a rectangle at the target cordinates with dimensions pixel_size,pixel_size
            draw_rectangle(base_x as f32 * pixel_size, base_y as f32 * pixel_size, pixel_size, pixel_size, color);
            curr_x += 1;
        }
        curr_x = 0;
        curr_y += 1;
    }
}

