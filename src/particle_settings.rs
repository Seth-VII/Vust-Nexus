use super::*;


// Predefined ParticleParams Settings for easier use and Overview inside the entities


pub fn thruster_settings( position: Vec2,dir: Vec2, color_tint: Color, spreading_range: Vec2 ) -> ParticleParams
{    

    let mut params : ParticleParams = ParticleParams { 
        spawn_rate: 0.5,
        spawn_count: 1,

        lifetime: 1.2,

        color_begin: WHITE, 
        color_end: color_tint,

        size_begin: vec2(10.0, 10.0), 
        render_scale: 1.0,

        use_velocity: true,
        spawn_position: position, 
        velocity: dir, 
        
        speed_begin: 3.0,
        speed_end: 1.0,
        ..Default::default()
    };
    params.randomize_velocity( 
        vec2(0.0, 2.0),
        spreading_range
    );

    params
}


pub fn fire_settings( position: Vec2,dir: Vec2  ) -> ParticleParams
{
    // | Right X | Left -X |  Bottom Y | Top -Y |
    let dir_multiplier = vec2(50.0, 50.0) + (vec2(50.0, 50.0) * dir);
    //let mut range = vec4(0.0 , 0.0, 0.0, 0.0);
    let mut range = vec4(0.0,0.0,0.0,0.0);
    
    if dir.x > 0.0
    {
        //range = vec4(100.0, 80.0, 50.0, -50.0);
        range = vec4( dir_multiplier.x * 2.0, dir_multiplier.x , dir_multiplier.y, dir_multiplier.y * -1.0);
    }
    if dir.x < 0.0
    {
        //range = vec4(-80.0, -100.0, 50.0, -50.0);
        range = vec4( dir_multiplier.x * -2.0, dir_multiplier.x , dir_multiplier.y, dir_multiplier.y * -1.0);
    }
    if dir.y > 0.0
    {
        //range = vec4(50.0, -50.0, 100.0, 80.0);
        range = vec4( dir_multiplier.x, dir_multiplier.x * -1.0, dir_multiplier.y * 2.0, dir_multiplier.y);
    }
    if dir.y < 0.0
    {
        //range = vec4(50.0, -50.0, -80.0, -100.0);
        range = vec4( dir_multiplier.x, dir_multiplier.x * -1.0, dir_multiplier.y * -2.0, dir_multiplier.y);
    }
    

    let mut params : ParticleParams = ParticleParams { 
        spawn_rate: 2.0,
        spawn_count: 4,

        lifetime: 1.0,

        color_begin: WHITE, 
        color_end: color_u8!(0,0,0,0), 

        size_begin: vec2(5.0, 5.0), 
        size_end: vec2(0.0, 0.0), 
        render_scale: 1.0,

        //position_random_range: range,
        
        use_velocity: true,
        spawn_position: position, 
        velocity: dir, 

        speed_begin: 5.0,
        speed_end: 1.0,
        ..Default::default()
    };

    params.randomize_velocity( 
        vec2(0.0, 2.0),
        vec2(-10.0, 10.0)
    );
    params
}


pub fn explosion_settings( color_1: Color,  color_2: Color , color_end: Color ) -> ParticleParams
{
    // XMin , XMax , YMin , YMax
    // | Right X | Left -X |  Bottom Y | Top -Y |
    let range = vec4(-500.0 , 500.0, -500.0, 500.0);

    let mut params : ParticleParams = ParticleParams { 
        spawn_rate: 500.0,
        spawn_count: 20,

        lifetime: 1.0,

        color_begin: WHITE, 
        color_end: color_end, 

        size_begin: vec2(15.0, 15.0), 
        size_end: vec2(0.0, 0.0), 
        render_scale: 1.0,

        position_begin: vec2(0.0, 0.0), 
        position_end: vec2(0.0, 0.0), 
        position_random_range: range,
        position_random:  vec2(0.0, 0.0),

        use_velocity: false,
        spawn_position: vec2(0.0, 0.0), 
        velocity: vec2(0.0, 0.0), 

        rotation_begin: 0.0, 
        rotation_end: 1.0,
        
        speed_begin: 6.0,
        speed_end: 1.0,
    };

    params.randomize_color(color_1, color_2);
    params
}

