use super::*;

#[derive(PartialEq, Debug)]
pub enum GameState 
{ GameRunning, GamePaused, MainMenu, GameOver, LevelCompleted, Transition}