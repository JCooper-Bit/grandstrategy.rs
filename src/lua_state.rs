use std::sync::Arc;
use std::{sync::Mutex};
use bevy::prelude::*;
use mlua::{Lua, Function};

#[derive(Debug)]
pub enum LuaCommand {
    Schedule {
        id: String,
        day: u32,
        month: u32,
        year: u32,
        callback: String,
    },
    Cancel {
        id: String,
    },
    ScheduleInNDays {
        id: String,
        n: u32,
        callback: String
    }
}



#[derive(Resource, Clone)]
pub struct LuaState {
    pub lua: Lua,
    pub schedule_commands: Arc<Mutex<Vec<LuaCommand>>>,
}

impl LuaState {
    /// Create Lua + inject API + load scripts
    pub fn new() -> Self {
        let lua = Lua::new();
        let schedule_commands = Arc::new(Mutex::new(Vec::<LuaCommand>::new()));
    
        let clua = lua.clone();
        let state = Self {lua: clua, schedule_commands};
        // 1. Inject Game API FIRST
        state.init_game_api(state.clone().schedule_commands);
        // 2. Load Lua scripts AFTER API exists
        state.load_script(&lua, "game/main.lua");

        state        

    }


    pub fn load_script(&self, lua: &Lua, path: &str) {
    
        let code = std::fs::read_to_string(path)
            .expect("missing Lua script");

        lua.load(&code)
            .exec()
            .expect("Lua script error");
    }

    fn init_game_api(&self, commands: Arc<Mutex<Vec<LuaCommand>>>) {
        let globals = self.lua.globals();
        let game = self.lua.create_table().unwrap();
        let events = self.lua.create_table().unwrap();
 
        let schedule_commands = commands.clone();

        let schedule = self.lua.create_function(
            move |_lua, (day, month, year, callback): (u32, u32, u32, String)| {
                let mut cmds = schedule_commands.lock().unwrap();

                let id = format!(
                    "{}_{}",
                    callback,
                    uuid::Uuid::new_v4().simple()
                );
                let idc = id.clone();
                cmds.push(LuaCommand::Schedule {
                    id,
                    day,
                    month,
                    year,
                    callback,
                });
                Ok((idc))
            },
        ).unwrap();

        let log = self.lua
            .create_function(|_, msg: String| {
                println!("[Lua] {msg}");
                Ok(())
            }).unwrap();


        let cancel_commands = commands.clone();

        let cancel = self.lua.create_function(
            move |_, id: String| {
                let mut cmds = cancel_commands.lock().unwrap();

                cmds.push(LuaCommand::Cancel { id });

                Ok(())
            },
        ).unwrap();

        let sched_n_days_commands = commands.clone();

        let schedule_in_n_days = self.lua.create_function(move |_lua, (n, callback): (u32, String)| {
            let mut cmds = sched_n_days_commands.lock().unwrap();

            let id = format!(
                "{}_{}",
                callback,
                uuid::Uuid::new_v4().simple()
            );
            let idc = id.clone();
            cmds.push(LuaCommand::ScheduleInNDays {id, n, callback});
            Ok((idc))
        }).unwrap();

        game.set("log", log).unwrap();

        events.set("schedule", schedule).unwrap();
        events.set("schedule_in_n_days", schedule_in_n_days).unwrap();
        events.set("cancel", cancel).unwrap();
        
        globals.set("Game", game).unwrap();
        globals.set("Events", events).unwrap();
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
