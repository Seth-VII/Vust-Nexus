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
    pub fn apply_state_setup( &mut self , state_setup: StateMachineSetup ) {
        self.state_machine.animation_states = state_setup.animation_states;
        self.state_machine.apply_animation_sheet( self.texture_sheet_size);
    }
    pub fn get_statemachine_mut(&mut self) -> &mut StateMachine {&mut self.state_machine}
    pub fn setup_sheet_size(&mut self,x: usize, y: usize) {
        self.texture_sheet_size = (x,y);
        
    }
    pub fn play_anim_once(&mut self) {
        self.state_machine.play();
    }
    pub fn update(&mut self) { 
        self.state_machine.update();
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

    pub fn play_anim_loop(&mut self, start_frame: usize)
    {
        self.is_playing = true;
        self.frame_t = self.duration;
        self.alpha = 1.0;
        self.current_frame = self.get_sheet_position(start_frame);
    }

    pub fn stop(&mut self)
    {
        self.is_playing = false;
        self.alpha = 0.0;
        self.frame_t = 0.0;
        let last_frame = self.texture_sheet_size.0 * self.texture_sheet_size.1;
        self.current_frame = self.get_sheet_position(last_frame);

    }
    pub fn stop_slice(&mut self, end_frame: usize)
    {
        self.is_playing = false;
        self.alpha = 0.0;
        self.frame_t = 0.0;
        let last_frame = end_frame;
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

        }else if self.loop_anim && self.frame_t <= 0.0 {
            self.stop();
            //self.play_anim_loop();
        }else {
            self.stop();
        }

    }

    pub fn update_slice(&mut self, start_frame: usize, end_frame: usize)
    {
        if self.frame_t > 0.0
        {
            self.frame_t -= 1.0 * self.anim_speed * get_frame_time();
            self.alpha = self.frame_t;
            if self.frame_t < 0.0 { self.frame_t = 0.0; self.alpha = 0.0;}
            
            //println!("alpha: {}", self.alpha);
            
            let position = f32::lerp(
                &(end_frame as f32) , 
                &(start_frame as f32), 
                &self.alpha
            );
            
            self.current_frame = self.get_sheet_position(position.ceil() as usize);
            //println!("frame: {:?}", self.current_frame);
            
        }else if self.loop_anim  && self.frame_t <= 0.0 {
            //println!("time {}", self.frame_t);
            self.play_anim_loop(start_frame);
        }else {
            self.stop_slice(end_frame);
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
    speed: f32,

    animation: Option<Animation>,
}
impl AnimationState
{
    pub fn new( start: usize, end: usize, is_looping: bool, speed: f32 ) ->  Self
    {
        Self { frame_start: start, frame_end: end, loop_anim: is_looping, is_playing: false, speed: speed, animation: None}
    }
    pub fn apply_animation(&mut self,  sheet: (usize, usize)) { 
        let mut animation = Animation::new(sheet);
        animation.set_animation_speed(self.speed);
        self.animation =  Some(animation);
    }
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
    pub fn play(&mut self)
    {
        if self.animation.is_none() {return;}

        if self.animation.as_ref().unwrap().loop_anim
        {
            if !self.animation.as_ref().unwrap().is_playing
            {
                self.animation.as_mut().unwrap().play_anim_loop(self.frame_start);
            }
        }else {
            self.animation.as_mut().unwrap().play_anim_once();
        }
    }
    pub fn reset(&mut self)
    {
        if self.animation.is_none() {return;}
        self.animation.as_mut().unwrap().stop();
    }
    pub fn update_animation(&mut self) { 
        if self.animation.is_none() {return;}
        
        self.animation.as_mut().unwrap().update_slice(self.frame_start,self.frame_end);
        self.animation.as_mut().unwrap().loop_anim = self.loop_anim;
        //println!("Animation frame {}", self.animation.as_mut().unwrap().get_current_frame());

    }
    pub fn get_current_animation_frame(&self) -> Vec2 {
        if self.animation.is_none() {return vec2(0.0, 0.0);}
        self.animation.as_ref().unwrap().get_current_frame()
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
        let mut default_state = AnimationState::new(start, end, false, 1.0);
        default_state.apply_animation(sheet);
        Self { animation_states: vec![default_state; 1], current_state: 0, sheet: sheet }
    }
    
    pub fn apply_animation_sheet(&mut self, sheet: (usize, usize))
    {
        for state in self.animation_states.iter_mut()
        {
            state.apply_animation(sheet);
        } 
    }

    pub fn play(&mut self)
    {
        self.animation_states[self.current_state].play();
    }
    pub fn update(&mut self) { 
        self.animation_states[self.current_state].update_animation();
    }


    pub fn get_current_frame(&self) -> Vec2 {self.animation_states[self.current_state].get_current_animation_frame()}

    pub fn AddStateMachine(&mut self, state: AnimationState) { self.animation_states.push(state);}
    
    pub fn SetState(&mut self, to_state: usize ) {
        if self.current_state != to_state
        {

            self.animation_states[ self.current_state ].reset();
            
            self.current_state = to_state;
            self.animation_states[self.current_state].reset();
            self.animation_states[self.current_state].play();
        }
    }
    
}


pub struct StateMachineSetup {
    pub animation_states: Vec<AnimationState>,
    pub default_state: usize,
}
impl StateMachineSetup {
    pub fn new () -> Self{ Self { animation_states: Vec::new(), default_state: 0,}}

    pub fn player_setup()  -> Self
    {
        let mut states = Vec::new();

        let idle = AnimationState::new(0,20, true, 5.0);
        //let down = AnimationState::new(4,7, false, 25.0);
        //let up = AnimationState::new(8,11, false, 25.0);

        states.push(idle);
        //states.push(down);
        //states.push(up);

        Self { animation_states: states, default_state: 0}
    }
    
    pub fn spawner_setup()  -> Self
    {
        let mut states = Vec::new();

        let grow = AnimationState::new(0,7, false, 0.5);
        let spawn = AnimationState::new(4,7, true, 10.0);
        //let up = AnimationState::new(8,11, false, 25.0);

        states.push(grow);
        states.push(spawn);
        //states.push(up);

        Self { animation_states: states, default_state: 1}
    }
    
    //pub fn get_statemachine(&self) -> &StateMachine {&self.statemachine}
}

