use bevy::prelude::*;
use mlua::{Lua, Table, Function, Result};

/// IMPORTANT:
/// Use this as a NON-SEND resource in Bevy
/// (Lua is not thread-safe)
#[derive(Resource)]
pub struct LuaState {
    pub lua: Lua,
}

impl LuaState {
    /// Create Lua + inject API + load scripts
    pub fn new() -> Self {
        let lua = Lua::new();
        let clua = lua.clone();
        let mut state = Self {lua: clua};
        // 1. Inject Game API FIRST
        state.init_game_api();
        // 2. Load Lua scripts AFTER API exists
        state.load_script(&lua, "game/main.lua");

        state        

    }

    // pub fn on_month_tick(&self, day: u32, month: u32, year: u32) {
    //     let globals = self.lua.globals();

    //     let game: mlua::Table = match globals.get("Game") {
    //         Ok(g) => g,
    //         Err(_) => return,
    //     };

    //     let func: mlua::Result<mlua::Function> = game.get("on_month_tick");

    //     if let Ok(f) = func {
    //         let _ = f.call::<()>((day, month, year));
    //     }
    // }

    pub fn load_script(&self, lua: &Lua, path: &str) {
    
        let code = std::fs::read_to_string(path)
            .expect("missing Lua script");

        lua.load(&code)
            .exec()
            .expect("Lua script error");
    }

    fn init_game_api(&self) {
    let globals = self.lua.globals();

    let game = self.lua.create_table().unwrap();


    let log = self.lua
        .create_function(|_, msg: String| {
            println!("[Lua] {msg}");
            Ok(())
        })
        .unwrap();

    game.set("log", log).unwrap();



    globals.set("Game", game).unwrap();
    }
    
    pub fn call_game_function(&self, name: &str, args: impl mlua::IntoLuaMulti) {
        let globals = self.lua.globals();

        let game: mlua::Table = match globals.get("Game") {
            Ok(g) => g,
            Err(_) => return,
        };

        let func: mlua::Result<Function> = game.get(name);

        if let Ok(f) = func {
            let _ = f.call::<()>(args);
        }
    }
}
