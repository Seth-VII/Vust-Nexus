use super::*;
use interpolation::*;

#[derive(Clone)]
pub struct Animation
{
    pub is_playing: bool,
    
    texture_sheet_size: (usize, usize),

    current_frame: Vec2,
    duration: f32,
    anim_speed: f32,

    frame_t: f32,
    alpha: f32,
}
impl Animation
{
    pub fn new(sheet: (usize, usize)) -> Self {
        Self { 
            is_playing: false,

            texture_sheet_size: sheet,
            current_frame: vec2(0.0, 0.0),
            duration: 1.0,
            anim_speed: 1.0,


            frame_t: 1.0,
            alpha: 1.0,

        }
    }

    // Sprite Sheet Animation
    pub fn get_current_frame(&self) -> Vec2 {self.current_frame}
    pub fn set_animation_duration(&mut self, duration: f32) {self.duration = duration;}
    pub fn set_animation_speed(&mut self, speed: f32) {self.anim_speed = speed;}

    pub fn setup_sheet_size(&mut self,x: usize, y: usize) {self.texture_sheet_size = (x,y);}

    pub fn play_anim_once(&mut self)
    {
        if !self.is_playing
        {
            self.is_playing = true;
            self.frame_t = self.duration;
            self.alpha = 1.0;
        }
    }
    pub fn stop(&mut self)
    {
        self.is_playing = false;
    }
    pub fn update(&mut self)
    {
        if self.frame_t > 0.0
        {

            self.frame_t -= 1.0 * self.anim_speed * get_frame_time();
            self.alpha = self.frame_t;
            
            //println!("alpha: {}", self.alpha);

            let position = f32::lerp(
                &(self.texture_sheet_size.0 as f32 * self.texture_sheet_size.1 as f32) , 
                &0.0, 
                &self.alpha
            );
            
            self.current_frame = self.get_sheet_position(position.ceil() as usize);
            //println!("frame: {:?}", self.current_frame);

        }else {
            self.stop();
        }

    }
    fn get_sheet_position(&self, position: usize) -> Vec2
    {
        let mut x = 0;
        let mut y = 0;
        for i in 0..position
        {
            x += 1;
            if x >= self.texture_sheet_size.0 
            {
                x = 0;
                y += 1;
                if y >= self.texture_sheet_size.1
                {
                    x = 0;
                    y = 0;
                }
            }
        }
        vec2(x as f32, y as f32)
    }

   
}