extern crate sdl2;
extern crate rand;

mod sprite;
mod timer;
mod engine;
mod alien_sprite;
mod background;
mod spaceout;

use std::time::{Duration, SystemTime};
use std::{thread, time};
use rand::Rng;
use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::mem::transmute;
use std::collections::HashMap;
use sdl2::render::{ WindowCanvas, Texture };
use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::render::TextureQuery;
use sdl2::mixer::{ Music, Chunk, AUDIO_S16LSB };

use spaceout::{ SpaceOut, CLIENT_WIDTH, CLIENT_HEIGHT, EVENT_MOUSE_CLICK, EVENT_MOUSE_MOVE };

use spaceout::{
    RES_SPLASH_BITMAP,
    RES_DESERT_BITMAP,
    RES_CAR_BITMAP,
    RES_SM_CAR_BITMAP,
    RES_MISSILE_BITMAP,
    RES_BLOBBO_BITMAP,
    RES_BMISSILE_BITMAP,
    RES_JELLY_BITMAP,
    RES_JMISSILE_BITMAP,
    RES_TIMMY_BITMAP,
    RES_TMISSILE_BITMAP,
    RES_SM_EXPLOSION_BITMAP,
    RES_LG_EXPLOSION_BITMAP,
    RES_GAME_OVER_BITMAP,

    RES_BMISSILE_SOUND,
    RES_GAMEOVER_SOUND,
    RES_JMISSILE_SOUND,
    RES_LG_EXPLODE_SOUND,
    RES_SM_EXPLODE_SOUND,
    RES_MISSILE_SOUND,
};

//下载SDL_image
//https://www.libsdl.org/projects/SDL_image/
//ttf
//https://www.libsdl.org/projects/SDL_ttf/
static mut WINDOW:*const Window = 0 as *const Window;

//获取全局的SpaceOut实例
fn window<'a>() -> &'a mut Window<'a> {
    unsafe {
        if WINDOW.is_null() {
            WINDOW = transmute(Box::new(Window::new()));
        }
        transmute(WINDOW)
    }
}

struct Window<'a>{
    game: SpaceOut,
    sdl_context: Sdl,
    font: Option<Font<'a, 'a>>,
    canvas:WindowCanvas,
    resources:Option<&'a HashMap<i32, Texture<'a>>>,
    sounds:HashMap<i32, Chunk>,
    music: Option<Music<'a>>,
    
}

impl <'a>Window<'a>{
    fn new()->Window<'a>{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("SpaceOut", CLIENT_WIDTH as u32, CLIENT_HEIGHT as u32)
            .position_centered().build().unwrap();

        let canvas = window.into_canvas()
            .accelerated().build().unwrap();
        Window{
            game: SpaceOut::new(),
            sdl_context: sdl_context,
            font: None,
            canvas: canvas,
            resources: None,
            sounds: HashMap::new(),
            music: None
        }
    }

    fn run(&mut self){
        //创建游戏
        self.game.new_game();

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let mut running = true;
        let one_millis = time::Duration::from_millis(1);
        let mut start_time = SystemTime::now();
        while running {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                        running = false;
                    },
                    Event::MouseButtonDown{x, y, ..} => {
                        self.game.on_touch_event(EVENT_MOUSE_CLICK, x, y);
                    },
                    Event::MouseMotion{x, y, ..} => {
                        self.game.on_touch_event(EVENT_MOUSE_MOVE, x, y);
                    }
                    _ => {}
                }
            }
            if self.game.engine().ready_for_next_frame(){
                self.canvas.clear();
                self.game.game_cycle();

                //绘制帧率
                /*
                let elapsed = start_time.elapsed().unwrap();
                let nanos = elapsed.as_secs()*1000000000+elapsed.subsec_nanos() as u64;
                if nanos>0{
                    draw_text(&format!("FPS:{}", 1000000000/nanos), 10, 10);
                }
                start_time = SystemTime::now();
                */
                self.canvas.present();
            }
            //给一些延迟, CPU使用率降低到1.5%左右
            thread::sleep(one_millis);
        }
    }

    fn get_font(&self)->&Font{
        self.font.as_ref().unwrap()
    }
}

fn main() {
    
    //初始化音乐
    let _audio = window().sdl_context.audio().unwrap();
    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = 2; // Stereo
    let chunk_size = 1024;
    match sdl2::mixer::open_audio(frequency, format, channels, chunk_size){
        Ok(()) => (),
        Err(err) =>{
            println!("音频初始化失败 {:?}", err);
        }
    }

    sdl2::mixer::allocate_channels(6);

    println!("加载字体...");
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(Path::new("resources/SourceHanSerifCN-Light.otf"), 20).unwrap();
    let texture_creator = window().canvas.texture_creator();
    
    //加载资源
    let mut resources = HashMap::new();
    
    let load_png = |file|->Texture{
        let rwops = RWops::from_file(Path::new(file), "r").unwrap();
        let texture = texture_creator.create_texture_from_surface(&rwops.load_png().unwrap()).unwrap();
        println!("加载:{}", file);
        texture
    };

    window().font = Some(font);

    resources.insert(RES_SPLASH_BITMAP, load_png("resources/Splash.png"));
    resources.insert(RES_DESERT_BITMAP, load_png("resources/Desert.png"));
    resources.insert(RES_CAR_BITMAP, load_png("resources/Car.png"));
    resources.insert(RES_SM_CAR_BITMAP, load_png("resources/SmCar.png"));
    resources.insert(RES_MISSILE_BITMAP, load_png("resources/Missile.png"));
    resources.insert(RES_BLOBBO_BITMAP, load_png("resources/Blobbo.png"));
    resources.insert(RES_BMISSILE_BITMAP, load_png("resources/BMissile.png"));
    resources.insert(RES_JELLY_BITMAP, load_png("resources/Jelly.png"));
    resources.insert(RES_JMISSILE_BITMAP, load_png("resources/JMissile.png"));
    resources.insert(RES_TIMMY_BITMAP, load_png("resources/Timmy.png"));
    resources.insert(RES_TMISSILE_BITMAP, load_png("resources/TMissile.png"));
    resources.insert(RES_SM_EXPLOSION_BITMAP, load_png("resources/SmExplosion.png"));
    resources.insert(RES_LG_EXPLOSION_BITMAP, load_png("resources/LgExplosion.png"));
    resources.insert(RES_GAME_OVER_BITMAP, load_png("resources/GameOver.png"));

    let load_sound = |res_id, file|{
        match Chunk::from_file(Path::new(file)){
            Ok(music) => {
                window().sounds.insert(res_id, music);
                println!("加载:{}", file);
            }
            Err(err) =>{
                println!("加载:{} {:?}", file, err);
            }
        }
    };

    load_sound(RES_BMISSILE_SOUND, "resources/BMissile.wav");
    load_sound(RES_GAMEOVER_SOUND, "resources/GameOver.wav");
    load_sound(RES_JMISSILE_SOUND, "resources/JMissile.wav");
    load_sound(RES_MISSILE_SOUND, "resources/Missile.wav");
    load_sound(RES_LG_EXPLODE_SOUND, "resources/LgExplode.wav"); 
    load_sound(RES_SM_EXPLODE_SOUND, "resources/SmExplode.wav");
    
    if let Ok(music) = Music::from_file(Path::new("resources/Music.mp3")){
        window().music = Some(music);
    }
    
    window().resources = Some(&resources);
    window().run();
}

fn fill_style_rgb(r:u8, g:u8, b:u8){
    window().canvas.set_draw_color(Color::RGB(r, g, b));
}

fn fill_rect(x:i32, y:i32, width:i32, height:i32){
    window().canvas.fill_rect(Rect::new(x, y, width as u32, height as u32)).unwrap();
}

fn draw_image_at(res_id:i32, x:i32, y:i32){
    let mut texture = window().resources.as_ref().unwrap().get(&res_id).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    window().canvas.copy(&mut texture, None, Some(Rect::new(x, y, width, height))).unwrap();
}

fn draw_image(res_id:i32, source_x:i32, source_y:i32, source_width:i32, source_height:i32, dest_x:i32, dest_y:i32, dest_width:i32, dest_height:i32){
    let mut texture = window().resources.as_ref().unwrap().get(&res_id).unwrap();
    window().canvas.copy(&mut texture, 
        Some(Rect::new(source_x, source_y, source_width as u32, source_height as u32)),
        Some(Rect::new(dest_x, dest_y, dest_width as u32, dest_height as u32))
    ).unwrap();
}

fn draw_text(text: &str, x:i32, y:i32){
    let surface = window().get_font().render(text).solid(Color::RGB(255, 255, 255)).unwrap();
    let texture_creator = window().canvas.texture_creator();
    let mut texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    window().canvas.copy(&mut texture, None, Some(Rect::new(x, y, width, height))).unwrap();
}

pub fn current_time()->f64{
    window().game.engine().elapsed_secs()
}

//返回[low, low] 区间的数
pub fn rand_int(low: i32, high: i32) -> i32{
    rand::thread_rng().gen_range(low, high+1)
}

pub fn random()->f64{
    rand::thread_rng().next_f64()
}

pub fn play_sound(res_id:i32){
    if let Some(chunk) = window().sounds.get(&res_id){
        match sdl2::mixer::Channel::all().play(&chunk, 1){
            Ok(_) => (),
            _ => ()
        }
    }
}

pub fn play_music(_url:&str){
    if let Some(music) = window().music.as_ref(){
        music.play(100).unwrap();
    }
}

pub fn pause_music(){
    Music::pause();
}