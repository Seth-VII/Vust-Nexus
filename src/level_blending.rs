use super::*;
use interpolation::*;
#[derive(PartialEq, Debug)]
pub enum BlendingType {BlendIn, BlendOut, BlackDelay}
pub struct BlackBlend
{
    is_blending: bool,
    blendtype: BlendingType,
    speed: f32,
    alpha: f32,
    delay: f32,
    delayed_time: f32,
}
impl BlackBlend
{
    pub fn get_is_playing(&self) -> bool {self.is_blending}
    pub fn set_start_blend(&mut self, b_type: BlendingType, alpha: f32) {
        self.blendtype = b_type; 
        self.alpha = alpha;
    }
    pub fn blend_in(&mut self)
    {
        self.blendtype = BlendingType::BlendIn;
    }
    pub fn blend_out(&mut self)
    {
        self.blendtype = BlendingType::BlendOut;
    }
    pub fn update_blend(&mut self, level_offset: f32)
    {
        match self.blendtype{
            BlendingType::BlendIn => {

                if self.alpha < 1.0
                {
                    self.alpha += self.speed * get_frame_time();
                    self.is_blending = true;
                }else {
                    self.alpha = 1.0;
                    self.blendtype = BlendingType::BlackDelay;
                    //self.delayed_time = 0.0;
                }

            }
            BlendingType::BlackDelay => {
                self.alpha = 1.0;
                if self.delayed_time < self.delay
                {
                    self.delayed_time += self.speed * get_frame_time();
                }else {
                    self.delayed_time = 1.0;
                    self.is_blending = false;
                }
                println!("{}", self.delayed_time);
            }
            BlendingType::BlendOut => {
                if self.alpha > 0.0
                {
                    self.alpha -= self.speed * get_frame_time();
                    self.is_blending = true;
                }else {
                    self.alpha = 0.0;
                    self.is_blending = false;
                }

            }
        }

        let color = Color::from_vec(
            Vec4::lerp(
                vec4(0.0 ,0.0 ,0.0 ,0.0),
                vec4(0.0 ,0.0 ,0.0 ,1.0),
                self.alpha
            )
        );

        //println!("{:?}", color);
        draw_rectangle(level_offset, 0.0, GAME_SIZE_X as f32, GAME_SIZE_Y as f32, color);
    }
}
impl Default for BlackBlend
{
    fn default() -> Self {
        Self { speed: 0.8, alpha: 0.0, blendtype: BlendingType::BlendOut, is_blending: false, delay: 1.0, delayed_time: 0.0 }
    }
}