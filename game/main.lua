Game.log("Game Start.")

Game.on_year_tick = function(day, month, year)
    Game.log("Year tick: " .. day .. "-" .. month .. "-" .. year)
end

Game.on_month_tick = function(day, month, year)
    Game.log("Month tick: " .. day .. "-" .. month .. "-" .. year)
end

Game.on_day_tick = function(day, month, year)
    Game.log("Day tick: " .. day .. "-" .. month .. "-" .. year)
end

local invasion_id = Events.schedule(10, 1, 2026, "on_invasion")
local wedding_id = Events.schedule(5, 2, 2026, "wedding")
Game.on_invasion = function(d,m, y) 
    Game.log("Invasion Triggered")
    Events.cancel(wedding_id)



end