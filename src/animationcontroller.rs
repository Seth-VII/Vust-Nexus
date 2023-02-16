use super::*;
use interpolation::*;
#[derive(Clone)]
pub struct AnimationController
{
    texture_sheet_size: (usize, usize),
    state_machine: StateMachine,
}
impl AnimationController
{
    pub fn new(sheet: (usize, usize)) -> Self {
        Self { 
            texture_sheet_size: sheet,
            state_machine: StateMachine::default(0, sheet.0 * sheet.1, sheet),
        }
    }
    pub fn setup_sheet_size(&mut self,x: usize, y: usize) {self.texture_sheet_size = (x,y);}
    pub fn play_anim_once(&mut self)
    {
        self.state_machine.play();
    }

    pub fn get_current_frame(&self) -> Vec2 {self.state_machine.get_current_frame()}
}


#[derive(Clone)]
pub struct Animation
{
    pub is_playing: bool,
    loop_anim: bool,

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
            loop_anim: false,

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

    pub fn play_anim_loop(&mut self)
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
        let last_frame = self.texture_sheet_size.0 * self.texture_sheet_size.1;
        self.current_frame = self.get_sheet_position(last_frame);
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

        }else if self.loop_anim {
            self.play_anim_loop();
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



#[derive(Clone)]
pub struct AnimationState
{
    frame_start: usize,
    frame_end: usize,
    loop_anim: bool,
    is_playing: bool,

    animation: Animation,
}
impl AnimationState
{
    pub fn new( start: usize, end: usize, is_looping: bool, sheet: (usize, usize) ) ->  Self
    {
        let animation = Animation::new(sheet);
        Self { frame_start: start, frame_end: end, loop_anim: is_looping, is_playing: false, animation: animation}
    }
    pub fn play(&mut self)
    {
        if self.animation.loop_anim
        {
            self.animation.play_anim_loop();
        }else {
            self.animation.play_anim_once();
        }
    }

}


#[derive(Clone)]
pub struct StateMachine
{
    pub animation_states: Vec<AnimationState>,
    current_state: usize,
    sheet: (usize, usize),
}
impl StateMachine
{
    pub fn new(sheet: (usize, usize)) ->  Self
    {
        Self { 
            animation_states: Vec::new(),
            current_state: 0,
            sheet: sheet,
        }
    }
    pub fn default(start: usize, end: usize, sheet: (usize, usize)) -> Self
    {
        let default_state = AnimationState::new(start, end, false, sheet);
        Self { animation_states: vec![default_state; 1], current_state: 0, sheet: sheet }
    }
    
    pub fn play(&mut self)
    {
        self.animation_states[self.current_state].play();
    }

    pub fn get_current_frame(&self) -> Vec2 {self.animation_states[self.current_state].animation.get_current_frame()}

    pub fn AddStateMachine(&mut self, state: AnimationState) { self.animation_states.push(state);}
    pub fn SwitchToState( to_state: usize ) {

    }
    
}


pub struct PlayerStateMachine { pub statemachine: StateMachine}
impl PlayerStateMachine {
    pub fn new ( sheet: (usize, usize) ) -> Self{ Self { statemachine: StateMachine::new(sheet) }}

    pub fn construct(&mut self)
    {
        let idle = AnimationState::new(0,4, true, self.statemachine.sheet);
        let down = AnimationState::new(5,9, true, self.statemachine.sheet);
        let up = AnimationState::new(10,14, true, self.statemachine.sheet);

        self.statemachine.AddStateMachine(idle);
        self.statemachine.AddStateMachine(down);
        self.statemachine.AddStateMachine(up);
    }


    pub fn get_statemachine(&self) -> &StateMachine {&self.statemachine}
}

