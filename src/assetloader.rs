
use std::fs::DirEntry;
use std::fs;
use super::*;

use macroquad::prelude::*;
use macroquad::audio::*;

#[derive(Clone,PartialEq)]
pub enum EAssetType
{
    Undefined, TextureFile, SoundFile
}
pub struct AssetLibrary
{
    pub assets: Vec<Asset>,
}
impl AssetLibrary 
{
    pub fn new() -> Self {
        Self { assets: Vec::new() }
    }
    pub async fn asset_loader_init(&mut self)
    {
        //self.init_load_executable("./resources/textures".to_string()).await;
        //self.init_load_executable("./resources/sounds".to_string()).await;
        self.init_load_for_wasm("resources/".to_string()).await;
    }
    pub async fn init_load_executable(&mut self, folder_path: String)
    {
        let dir_values = fs::read_dir(folder_path).unwrap();
        let mut path_vec : Vec<Box<DirEntry>> = Vec::new();

        for path in dir_values
        {
            let p = Box::new(path.unwrap());
            path_vec.push( p);
        }

        for path in path_vec
        {
            let mut new_asset = Asset::new( self, path.path().file_stem().unwrap().to_owned().into_string().unwrap());
        
            let mut asset_type = EAssetType::TextureFile;
            let mut display_type = "";

            if path.path().extension().unwrap().to_str().unwrap() == "png"{
                asset_type = EAssetType::TextureFile;
                display_type = "TextureFile";
            }
            if path.path().extension().unwrap().to_str().unwrap() == "wav"{
                asset_type = EAssetType::SoundFile;  
                display_type = "SoundFile";
            }
            println!("LOAD NEW ASSET: ");
            new_asset.asset_type = asset_type.clone();
            new_asset.load_data(path.path().as_path().to_str().unwrap(), new_asset.asset_name.to_string().as_str(), asset_type).await;
            println!("Asset Name: {}", new_asset.asset_name);
            println!("Asset ID: {}", new_asset.asset_id);
            println!("Asset Path: {}", path.path().as_path().to_str().unwrap());
            println!("Asset Type: {}", display_type );
            
            self.assign_asset(new_asset.asset_id,new_asset);
        }

      
    }

    pub async fn init_load_for_wasm(&mut self, folder_path: String)
    {
        let path_dir = folder_path.as_str();
        let mut asset_paths : (Vec<String>,Vec<String>) = (Vec::new(), Vec::new());

        let files = vec![

            "enemy_1.png",
            "enemy_2.png",
            "enemy_3.png",
            "player.png",
            "Ship_sheet.png",
            "player_missle_1.png",
            "tile_texture_atlas.png",
            "weapon_sheet.png",

            "enemy_laserShoot_1.wav",
            "explosion_1.wav",
            "explosion_2.wav",
            "explosion_3.wav",
            "fire_1.wav",
            "hit_1.wav",
            "hurt_sound_1.wav",
            "laserShoot_1.wav",
            "laserShoot_2.wav",
            "pickup_sound_1.wav",
        ];

        for file in files
        {
            let mut full_path = "".to_string();
            full_path.push_str(path_dir);

            if file.contains(".png") {
                full_path.push_str("textures/");
            }
            if file.contains(".wav") {
                full_path.push_str("sounds/");
            }
            full_path.push_str(file);
            asset_paths.0.push(full_path);

            let mut name = file.to_string();
            let s = file.to_string();
            let offset = s.find(".").unwrap();
            name.drain(offset..);
            asset_paths.1.push(name);
        }

        for i in 0..asset_paths.0.len()
        {
            let path = asset_paths.0[i].clone();

            let mut new_asset = Asset::new( self, asset_paths.1[i].clone());
        
            let mut asset_type = EAssetType::TextureFile;
            let mut display_type = "";

            if path.contains(".png") {
                asset_type = EAssetType::TextureFile;
                display_type = "TextureFile";
            }
            if path.contains(".wav") {
                asset_type = EAssetType::SoundFile;  
                display_type = "SoundFile";
            }
            println!("LOAD NEW ASSET: ");
            new_asset.asset_type = asset_type.clone();
            println!("Asset Name: {}", new_asset.asset_name);
            println!("Asset ID: {}", new_asset.asset_id);
            println!("Asset Path: {}", path.as_str());
            println!("Asset Type: {}", display_type );

            new_asset.load_data(path.as_str(), new_asset.asset_name.to_string().as_str(), asset_type).await;
            
            self.assign_asset(new_asset.asset_id,new_asset);
        }
    }

    pub fn assign_asset(&mut self, index: usize,asset: Asset)
    {
        self.assets[index] = asset;
    }
    pub fn get_asset_by_id(&mut self, asset_id: usize) -> Asset
    {
        self.assets[asset_id].clone()
    }
    pub fn get_asset_by_name(&mut self, asset_name: String) -> Option<Asset>
    {
        for asset in self.assets.iter()
        {
            if asset.asset_name == asset_name
            {
                return Some(asset.clone());
            }
        }
        None
    }
    pub fn add_asset(&mut self, asset: Asset)
    {
        self.assets.push(asset);
    }
}



#[derive(Clone)]
pub struct Asset
{
    pub asset_id: usize,
    pub asset_name: String,
    pub asset_type: EAssetType,
    pub data: AssetData,
}
impl Asset {
    pub fn new(lib: &mut AssetLibrary, name: String) -> Self
    {
        let asset = Self { asset_id: lib.assets.len(), asset_name: name, asset_type: EAssetType::Undefined, data: AssetData::new() };
        lib.assets.push(asset.clone());
        asset
    }
    pub fn get_asset_data(&mut self) -> AssetData
    {
        self.data.clone()
    }
    pub fn get_texture_data(&mut self) -> Texture2D
    {
        match self.get_asset_data().texture_asset
        {
            Some(texture) => {texture.texture_data}
            None => {println!("No Texture Data Found!");Texture2D::empty()}
        }
    }
    pub fn get_texture_asset(&mut self) -> TextureAsset
    {
        match self.get_asset_data().texture_asset
        {
            Some(texture) => {texture}
            None => {println!("No Texture Data Found!");TextureAsset::new()}
        }
    }
    pub fn get_sound_data(&mut self) -> SoundData
    {
        match self.get_asset_data().sound_asset
        {
            Some(sound) => {sound.sound_data}
            None => {println!("No Texture Data Found!");SoundData::empty()}
        }
    }
    pub async fn load_data(&mut self, path: &str, filename: &str, asset_type: EAssetType)
    {
        match asset_type
        {
            EAssetType::Undefined => {},
            EAssetType::TextureFile => 
            {
                let mut texture_asset = TextureAsset::new();
                texture_asset.load(path, filename).await;
                self.data.texture_asset = Some(texture_asset);
            },
            EAssetType::SoundFile => 
            {
                let mut sound_asset = SoundAsset::new();
                sound_asset.load(path, filename,true).await;
                self.data.sound_asset = Some(sound_asset);
            }
        }
    }
}



#[derive(Clone)]
pub struct AssetData
{
    pub texture_asset: Option<TextureAsset>,
    pub sound_asset: Option<SoundAsset>,
}
impl AssetData 
{
    pub fn new() -> Self
    {
        Self { texture_asset: None, sound_asset: None}
    }
}

#[derive(Clone)]
pub struct TextureAsset
{
    pub texture_data: Texture2D,
    pub sheet_split: usize,

    pub grid: (usize, usize),
    pub animation: Animation,
    pub animation_controller: AnimationController,
}
impl TextureAsset 
{
    pub fn new() -> Self {
        let sheet_size = (1,1);
        Self { 
            texture_data: Texture2D::empty(),
            sheet_split: 0,
            grid: sheet_size,
            animation: Animation::new(sheet_size),
            animation_controller: AnimationController::new(sheet_size),
        }
    }
    async fn load(&mut self, path: &str, filename: &str)
    {
        path.to_string().push_str(filename);
        self.texture_data = load_texture(path.to_string().as_str()).await.unwrap();
        self.texture_data.set_filter(FilterMode::Nearest);
    }

    pub fn setup_sheet(&mut self ,x: usize, y: usize)
    {
        self.grid = (x,y);
        self.animation.setup_sheet_size(x, y);
        self.animation_controller.setup_sheet_size(x,y);
    }
    
    pub fn get_current_animation_frame(&self) -> Option<Rect>
    {
        let position = self.animation.get_current_frame();
        let rect = Rect::new(
            position.x * self.get_sheet_tile_size().x,
            position.y * self.get_sheet_tile_size().y,
            self.get_sheet_tile_size().x,
            self.get_sheet_tile_size().y
        );
        Some(rect)
    }
    pub fn get_current_anim_controller_frame(&self) -> Option<Rect>
    {
        let position = self.animation_controller.get_current_frame();
        let rect = Rect::new(
            position.x * self.get_sheet_tile_size().x,
            position.y * self.get_sheet_tile_size().y,
            self.get_sheet_tile_size().x,
            self.get_sheet_tile_size().y
        );
        Some(rect)
    }

    pub fn get_sheet_tile_size(&self) -> Vec2
    {
        let size_x = self.texture_data.width() / self.grid.0 as f32;
        let size_y = self.texture_data.height() / self.grid.1 as f32;
        vec2(size_x, size_y)
    }
    
}


#[derive(Clone)]
pub struct SoundData
{
    pub sound : Option<Sound>
}
impl SoundData 
{
    pub fn empty() -> Self { Self { sound: None }}
    pub fn new( sound_data: Sound ) -> Self { Self { sound: Some(sound_data) }}
}

#[derive(Clone)]
pub struct SoundAsset
{
    pub sound_data: SoundData,
}
impl SoundAsset 
{
    pub fn new() -> Self {Self { sound_data: SoundData::empty() }}
    async fn load(&mut self, path: &str, filename: &str, fullpath: bool)
    {
        if !fullpath
        {
            path.to_string().push_str(filename);
        } 
        let sound_data = SoundData::new(load_sound(path.to_string().as_str()).await.unwrap());
        self.sound_data = sound_data;
    }    
}

