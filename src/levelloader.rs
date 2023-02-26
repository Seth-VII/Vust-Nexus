use super::*;

#[derive(Clone)]
pub struct LoadedLevelData
{
    pub walls: Vec<Vec2>,
    pub blockingWalls: Vec<Vec2>,
    pub trapWalls: Vec<Vec2>,

    pub enemy_spawner: Vec<(Vec2, usize, usize)>,
    pub destructibles: Vec<Vec2>,
    pub turrets: Vec<Vec2>,
    pub level_end: Vec<Vec2>
}
impl LoadedLevelData
{
    pub fn new() -> Self { 
        Self { 
            walls: Vec::new(), 
            blockingWalls: Vec::new(), 
            trapWalls: Vec::new(), 
            enemy_spawner: Vec::new(), 
            destructibles: Vec::new(), 
            turrets: Vec::new(),
            level_end: Vec::new(),
        }
    }
}

pub struct LevelLoader
{
    pub levels: Vec<LoadedLevelData>,
}
impl LevelLoader 
{
    pub fn new() -> Self {
        Self { levels: Vec::new() }
    }
    pub async fn level_loader_init(&mut self)
    {
        self.init_load_for_wasm("resources/levels/".to_string()).await;
    }

    pub async fn init_load_for_wasm(&mut self, folder_path: String)
    {
        self.level_structure_files(&folder_path).await;
    }

    async fn level_structure_files(&mut self, folder_path: &String)
    {
        let path_dir = folder_path.as_str();

        let level_files = vec![
            //"test_level.png",
            //"test_level_2.png",
            "test_level_3.png",
            "Level_1.png",
            "Level_2.png",
        ];

        let spawnmap_files = vec![
            "test_level_3_SpawnMap.png",
            "Level_1_SpawnMap.png",
            "Level_2_SpawnMap.png",
        ];

        let level_paths = self.build_filepath(level_files, path_dir);
        let levelspawn_paths = self.build_filepath(spawnmap_files, path_dir);

        for i in 0..level_paths.0.len()
        {
            let path = level_paths.0[i].clone();
            let name = level_paths.1[i].clone();
            let level_image = self.load_file(path.as_str(), name.as_str()).await;
            let mut level_data = self.convert_image_to_level(&level_image);
            
            self.load_spawnmap(&mut level_data, &levelspawn_paths.0[i], &levelspawn_paths.1[i]).await;

            self.levels.push(level_data);
        }
    }
    fn build_filepath(&self ,files: Vec<&str>, path: &str) -> (Vec<String>,Vec<String>)
    {
        let mut file_paths : (Vec<String>,Vec<String>) = (Vec::new(), Vec::new());
        for file in files.iter()
        {
            let mut full_path = "".to_string();
            full_path.push_str(path);
            full_path.push_str(file);
            file_paths.0.push(full_path);

            let mut name = file.to_string();
            let s = file.to_string();
            let offset = s.find(".").unwrap();
            name.drain(offset..);
            file_paths.1.push(name);
        }
        return file_paths;
    }

    async fn load_spawnmap(&mut self, level_data: &mut LoadedLevelData, path: &str, name: &str)
    {
        path.to_string().push_str(name);
        let spawnmap_result = load_image(path.to_string().as_str()).await;
        match spawnmap_result
        {
            Ok(spawnmap) => {

                let mut found_spawner = 0;
                for y in 0..spawnmap.height()
                {
                    for x in 0..spawnmap.width()
                    {
                        if spawnmap.get_pixel(x as u32, y as u32).a == 1.0
                        {
                            
                            if found_spawner < level_data.enemy_spawner.len() {
                                // 1 = Count of Enemies
                                level_data.enemy_spawner[found_spawner] .1 = (spawnmap.get_pixel(x as u32, y as u32).g * 255.0) as usize;
                                // 2 = Type of Enemies
                                level_data.enemy_spawner[found_spawner] .2 = (spawnmap.get_pixel(x as u32, y as u32).b * 255.0) as usize;
                                println!("Spawner Count: {}", level_data.enemy_spawner[found_spawner] .1);
                                found_spawner += 1;
                            }
                        }
                    }
                } 

            }
            Err(error) => {}
        }

        
    }


    async fn load_file(&mut self, path: &str, filename: &str) -> Image
    {
        path.to_string().push_str(filename);
        let image = load_image(path.to_string().as_str()).await.unwrap();
        image
    }

    fn convert_image_to_level(&mut self, level_image: &Image) -> LoadedLevelData
    {
        let mut new_level = LoadedLevelData::new();

        let _wall = color_u8!(255,255,255,255);
        let _blocking_wall = color_u8!(180,180,180,255);
        let _trap_wall = color_u8!(255,1,128,255);
        let _destructible = color_u8!(0,0,255,255);
        let _enemyspawner = color_u8!(255,0,0,255);
        let _turret = color_u8!(255,255,0,255);
        let _level_end = color_u8!(0,255,0,255);

        for y in 0..level_image.height()
        {
            for x in 0..level_image.width()
            {
                let position = vec2(x as f32, y as f32);
                if level_image.get_pixel(x as u32, y as u32) == _wall
                {
                    new_level.walls.push( position );
                    
                }
                
                if level_image.get_pixel(x as u32, y as u32) == _blocking_wall
                {
                    new_level.blockingWalls.push( position );
                    
                }
                
                //println!("{:?}", level_image.get_pixel(x as u32, y as u32));
                //println!("{:?}", _trap_wall);
                // Color { r: 1.0, g: 0.0, b: 0.5019608, a: 1.0 }
                if level_image.get_pixel(x as u32, y as u32) == _trap_wall
                {
                    new_level.trapWalls.push( position );

                }
                if level_image.get_pixel(x as u32, y as u32) == _destructible
                {
                    new_level.destructibles.push( position );
                }
                if level_image.get_pixel(x as u32, y as u32) == _enemyspawner
                {
                    let spawner = (position, 0,0);
                    new_level.enemy_spawner.push( spawner );
                }
                if level_image.get_pixel(x as u32, y as u32) == _turret
                {
                    new_level.turrets.push( position );
                }
                if level_image.get_pixel(x as u32, y as u32) == _level_end
                {
                    new_level.level_end.push( position );

                }
            }
        } 
        new_level
    }
    
    
}
