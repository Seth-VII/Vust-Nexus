use super::*;

#[derive(Clone)]
pub struct LoadedLevelData
{
    pub walls: Vec<Vec2>,
    pub enemy_spawner: Vec<Vec2>,
    pub destructibles: Vec<Vec2>,
    pub turrets: Vec<Vec2>,
    pub level_end: Vec<Vec2>
}
impl LoadedLevelData
{
    pub fn new() -> Self { 
        Self { 
            walls: Vec::new(), 
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
        let path_dir = folder_path.as_str();
        let mut file_paths : (Vec<String>,Vec<String>) = (Vec::new(), Vec::new());

        let files = vec![

            //"test_level.png",
            //"test_level_2.png",
            "test_level_3.png",
            "Level_2.png",
        ];

        for file in files
        {
            let mut full_path = "".to_string();
            full_path.push_str(path_dir);
            full_path.push_str(file);
            file_paths.0.push(full_path);

            let mut name = file.to_string();
            let s = file.to_string();
            let offset = s.find(".").unwrap();
            name.drain(offset..);
            file_paths.1.push(name);
        }

        for i in 0..file_paths.0.len()
        {
            let path = file_paths.0[i].clone();
            let name = file_paths.1[i].clone();
            let level_image = self.load_file(path.as_str(), name.as_str()).await;
            let level_data = self.convert_image_to_level(&level_image);
            
            self.levels.push(level_data);
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

        let _wall = Color::new(1.0,1.0,1.0,1.0);
        let _destructible = Color::new(0.0,0.0,1.0,1.0);
        let _enemyspawner = Color::new(1.0,0.0,0.0,1.0);
        let _turret = Color::new(1.0,1.0,0.0,1.0);
        let _level_end = Color::new(0.0, 1.0, 0.0, 1.0);

        for y in 0..level_image.height()
        {
            for x in 0..level_image.width()
            {
                let position = vec2(x as f32, y as f32);
                if level_image.get_pixel(x as u32, y as u32) == _wall
                {
                    new_level.walls.push( position );

                }
                if level_image.get_pixel(x as u32, y as u32) == _destructible
                {
                    new_level.destructibles.push( position );
                }
                if level_image.get_pixel(x as u32, y as u32) == _enemyspawner
                {
                    new_level.enemy_spawner.push( position );
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
