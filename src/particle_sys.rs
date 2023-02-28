
use super::*;
use interpolation::*;
use macroquad::rand::*;

#[derive(Clone)]
pub struct ParticleSystemPool
{
    pool: Vec<ParticleSystem>,
}
impl ParticleSystemPool
{
    pub fn new() -> Self 
    {
        Self { pool: Vec::new()}
    }
    pub fn spawn_system_at_position(&mut self, position: Vec2, particle_count: usize, params: ParticleParams)
    {
        let mut particle_system = ParticleSystem::new(particle_count, params);
        particle_system.transform.position = position;
        particle_system.spawn_once();
        self.pool.push(particle_system);
    }

    pub fn spawn_constant_system_at_position(&mut self, position: Vec2, particle_count: usize, params: ParticleParams)
    {
        let mut particle_system = ParticleSystem::new(particle_count, params);
        particle_system.transform.position = position;
        particle_system.spawn_constant();
        self.pool.push(particle_system);
    }
    pub fn clear(&mut self) { self.pool.clear();}


    pub fn update(&mut self,world: &mut World)
    {
        self.pool.retain(|p| p.is_active == true);
        for ps in self.pool.iter_mut()
        {
            ps.update_particles(world);
        }
        //println!("PS_Pool Len: {}", self.pool.len());
    }
    pub fn draw(&mut self)
    {
        for ps in self.pool.iter_mut()
        {
            ps.draw_particles();
        }
    }
}

#[derive(Clone)]
pub struct ParticleSystem
{
    pub transform: Transform,
    
    pub pool: ParticlePool,
    
    pub params: ParticleParams,
    spawn_timer: f32,
    pub is_active: bool
}
impl ParticleSystem {
    pub fn new( pool_size: usize, params: ParticleParams) -> Self{


        Self { 
            transform: Transform::zero(),
            params: params,
            pool: ParticlePool::new(pool_size, params),
            spawn_timer: 1.0,
            is_active: true,
        }
    }
    pub fn set_particle_parameter( &mut self, params: ParticleParams)
    {
        self.params = params;
    }

    pub fn spawn_constant(&mut self)
    {
        if self.spawn_timer > 0.0
        {
            self.spawn_timer -= self.params.spawn_rate * 2.0 * get_frame_time();
            return;
        }
        self.spawn_timer = 1.0;
        for _i in 0..self.params.spawn_count 
        {
            self.params.randomize();
            self.pool.spawn_particle(self.params,self.transform);
        }
    }
    pub fn spawn_once(&mut self)
    {
        
        for _i in 0..self.params.spawn_count 
        {
            self.params.randomize();
            self.pool.spawn_particle(self.params,self.transform);
        }
    }
    pub fn update_particles(&mut self, world: &mut World)
    {
        if !self.is_active {return;}

        if self.pool.active_pool.len() > 0
        {
            self.pool.update(world);
        }else {
            self.is_active = false;
        }
    }
    pub fn draw_particles(&mut self)
    {
        if !self.is_active {return;}
        self.pool.draw();
    }
}

#[derive(Clone, Copy)]
pub struct ParticleParams
{
    pub spawn_rate: f32,
    pub spawn_count: usize,

    pub lifetime: f32,

    pub color_begin: Color,
    pub color_end: Color,

    pub size_begin: Vec2,
    pub size_end: Vec2,
    pub render_scale: f32,

    pub position_begin: Vec2,
    pub position_end: Vec2,
    pub position_random_range: Vec4, 
    pub position_random:  Vec2,

    pub use_velocity: bool,
    pub spawn_position: Vec2,
    pub velocity: Vec2,

    pub rotation_begin: f32,
    pub rotation_end: f32,

    pub speed_begin: f32,
    pub speed_end: f32,
}
impl ParticleParams {
    pub fn new() -> Self{ 
        Self { 
            spawn_rate: 1.0,
            spawn_count: 1,

            lifetime: 1.0,

            color_begin: BLACK, 
            color_end: color_u8!(0,0,0,0), 

            size_begin: vec2(1.0, 1.0), 
            size_end: vec2(0.0, 0.0), 
            render_scale: 5.0,

            position_begin: vec2(0.0, 0.0), 
            position_end: vec2(0.0, 0.0), 
            position_random_range: vec4(0.0, 0.0, 0.0, 0.0),
            position_random:  vec2(0.0, 0.0),

            use_velocity: false,
            spawn_position: vec2(0.0, 0.0), 
            velocity: vec2(0.0, 0.0), 
            
            rotation_begin: 0.0, 
            rotation_end: 1.0,

            speed_begin: 1.0,
            speed_end: 1.0,
        }
    }
    /*
    
    pub fn default() -> Self
    {
        Self { 
            spawn_rate: 1.0,
            spawn_count: 1,

            lifetime: 1.0,

            color_begin: WHITE, 
            color_end: color_u8!(0,0,0,0), 

            size_begin: vec2(10.0, 10.0), 
            size_end: vec2(0.0, 0.0), 
            render_scale: 5.0,

            position_begin: vec2(0.0, 0.0), 
            position_end: vec2(0.0, 0.0), 
            position_random_range: vec4(100.0, -100.0, 100.0, -100.0),
            position_random:  vec2(0.0, 0.0),

            use_velocity: false,
            spawn_position: vec2(0.0, 0.0), 
            velocity: vec2(0.0, 0.0), 

            rotation_begin: 0.0, 
            rotation_end: 1.0,
            
            speed_begin: 1.0,
            speed_end: 1.0,
        }
    }
     */
    pub fn set_color(&mut self, begin: Color, end: Color)
    {
        self.color_begin = begin;
        self.color_end = end;
    }
    pub fn set_position_randomrange(&mut self, range_x: Vec2, range_y: Vec2 )
    {
        self.position_random_range = vec4(range_x.x,range_x.y,range_y.x,range_y.y );
    }
    pub fn randomize(&mut self)
    {
        let value = vec2(RandomRange::gen_range(self.position_random_range.x, self.position_random_range.y), RandomRange::gen_range(self.position_random_range.z, self.position_random_range.w));
        self.position_random = vec2(value.x, value.y);

        /*
        let r = RandomRange::gen_range(0, 255);
        let g = RandomRange::gen_range(0, 255);
        let b = RandomRange::gen_range(0, 255);
        self.color_begin = color_u8!(r,g,b,255);
        let r = RandomRange::gen_range(0, 255);
        let g = RandomRange::gen_range(0, 255);
        let b = RandomRange::gen_range(0, 255);
        self.color_end = color_u8!(r,g,b,0);
        */
    }

    pub fn randomize_velocity(&mut self, range_x: Vec2, range_y: Vec2)
    {
        let value = vec2(RandomRange::gen_range(range_x.x, range_x.y), RandomRange::gen_range(range_y.x, range_y.y));
        self.velocity += vec2(value.x, value.y);
    }

    pub fn randomize_color(&mut self, color_1: Color, color_2: Color )
    {
        let color = Color::new( 
            RandomRange::gen_range(color_1.r, color_2.r),
            RandomRange::gen_range(color_1.g, color_2.g),
            RandomRange::gen_range(color_1.b, color_2.b),
            1.0
        );
        self.color_begin = color;

    }
    pub fn set_scale(&mut self, scale: f32)
    {
        self.render_scale = scale
    }
    pub fn set_position(&mut self, begin: Vec2, end: Vec2)
    {
        self.position_begin = begin;
        self.position_end = end;
    }
    pub fn set_size(&mut self, begin: Vec2, end: Vec2)
    {
        self.size_begin = begin * self.render_scale;
        self.size_end = end * self.render_scale;
    }
    pub fn set_rotation(&mut self, begin: f32, end: f32)
    {
        self.rotation_begin = begin;
        self.rotation_end = end;
    }
    pub fn set_speed(&mut self, speed: f32)
    {
        self.speed_begin = speed;
        self.speed_end = speed;
    }
}
impl Default for ParticleParams
{
    fn default() -> Self {
        Self { 
            spawn_rate: 1.0,
            spawn_count: 1,

            lifetime: 1.0,

            color_begin: WHITE, 
            color_end: color_u8!(0,0,0,0), 

            size_begin: vec2(10.0, 10.0), 
            size_end: vec2(0.0, 0.0), 
            render_scale: 5.0,

            position_begin: vec2(0.0, 0.0), 
            position_end: vec2(0.0, 0.0), 
            position_random_range: vec4(100.0, -100.0, 100.0, -100.0),
            position_random:  vec2(0.0, 0.0),

            use_velocity: false,
            spawn_position: vec2(0.0, 0.0), 
            velocity: vec2(0.0, 0.0), 

            rotation_begin: 0.0, 
            rotation_end: 1.0,
            
            speed_begin: 1.0,
            speed_end: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct ParticlePool
{
    pub pool: Vec<Particle>,
    pub active_pool: Vec<Particle>,
    pub display_count: usize,
}
impl ParticlePool
{
    pub fn new(count: usize, params: ParticleParams) -> Self
    {
        let mut new_pool = Vec::new();
        for i in 0..count
        {
            new_pool.push(Particle::new(i,params));
        }
        Self { pool: new_pool, display_count: 0, active_pool: Vec::new()}
    }
    pub fn spawn_particle(&mut self, params: ParticleParams, transform:Transform)
    {
        for particle in self.pool.iter_mut()
        {
            if particle.is_active == false
            {
                particle.set_start(params,transform);
                self.active_pool.push(particle.clone());
                return;
            }else {
                self.display_count = particle.id;
            }
        }
    }
    
    pub fn update(&mut self, world: &mut World)
    {
        self.active_pool.retain(|p| p.is_active == true);
        for i in self.active_pool.iter_mut()
        {
            i.update(world);
        }
    }
    pub fn draw(&mut self)
    {
        for i in self.active_pool.iter_mut()
        {
            i.draw();
        }
    }
}

#[derive(Clone, Copy)]
pub struct Particle
{
    id: usize,
    params: ParticleParams,
    lifetime: f32,
    transform: Transform,
    color: Color,

    alpha: f32,
    pub is_active: bool,
    in_view: bool,
}
impl Particle {
    pub fn new(id: usize,params: ParticleParams) -> Self
    {
        Self { 
            id: id,
            params: params,
            lifetime: params.lifetime,
            transform: Transform::zero(),
            color: params.color_begin,
            alpha: 0.0,
            is_active: false,
            in_view: false,
        }
    }
    pub fn set_start(&mut self, params: ParticleParams,transform: Transform)
    {
        self.params                 = params;
        self.is_active              = true;
        self.lifetime               = params.lifetime;

        self.params.position_begin  = params.position_begin + transform.position;
        self.params.position_end    = params.position_end + transform.position + params.position_random;

        self.params.size_begin      = params.size_begin + transform.size;
        self.params.size_end        = params.size_end + transform.size;

        if params.use_velocity
        {
            self.transform.position = params.spawn_position;
            self.params.velocity = params.velocity;
        }
    }
    pub fn check_screen_visibility(&mut self) -> bool
    {
        if  self.transform.get_centered_position().x > GAME_SIZE_X  ||
            self.transform.get_centered_position().x < 0.0 - self.transform.size.x ||
            self.transform.get_centered_position().y > GAME_SIZE_Y  ||
            self.transform.get_centered_position().y < 0.0 - self.transform.size.y
        {
            return false;
        }
        return true;
    }
    pub fn update(&mut self, world: &mut World)
    {

        if self.is_active == false
        {
            return;
        }
        if inside_windowborder(self.transform.rect, world.level_offset, 200.0) 
        {
            self.in_view = true;
        }else {
            self.in_view = false;
        }
        if self.lifetime > 0.01
        {
            
            self.lifetime -= 1.0 * self.params.speed_begin * get_frame_time();
            self.alpha = 1.0 - (( self.lifetime / self.params.lifetime)  );

            if self.params.use_velocity
            {
                self.transform.position += self.params.velocity;
            }else {
                
                self.transform.set_position( Vec2::lerp(
                    self.params.position_begin, 
                    self.params.position_end, 
                    self.alpha)
                );
            }


            self.transform.rotation = f32::lerp(
                &self.params.rotation_begin, 
                &self.params.rotation_end, 
                &self.alpha);
            self.transform.set_size( Vec2::lerp(
                self.params.size_begin * self.params.render_scale, 
                self.params.size_end * self.params.render_scale, 
                self.alpha)
            );
            /*
            self.speed = f32::lerp(
                &self.params.speed_begin, 
                &self.params.speed_end, 
                &self.alpha);
                */
            self.color = Color::from_vec(
                Vec4::lerp(
                    vec4(
                        self.params.color_begin.r, 
                        self.params.color_begin.g,
                        self.params.color_begin.b,
                        self.params.color_begin.a,
                    ),
                    vec4(
                        self.params.color_end.r, 
                        self.params.color_end.g,
                        self.params.color_end.b,
                        self.params.color_end.a,
                    ),
                    self.alpha
                )
            );
            
                
        }else {
            self.reset();
        }
    }

    pub fn draw(&mut self)
    {
        if self.is_active && self.in_view
        {
            draw_rectangle( 
                self.transform.position.x - (self.transform.size.x / 2.0), 
                self.transform.position.y - (self.transform.size.y / 2.0), 
                self.transform.size.x, 
                self.transform.size.y, 
                self.color);

        }else if self.is_active
        {
            draw_rectangle_lines( 
                self.transform.position.x - (self.transform.size.x / 2.0), 
                self.transform.position.y - (self.transform.size.y / 2.0), 
                self.transform.size.x, 
                self.transform.size.y, 
                1.0,
                self.color);
        }
    }

    pub fn reset(&mut self)
    {
        self.is_active = false;
    }
}
