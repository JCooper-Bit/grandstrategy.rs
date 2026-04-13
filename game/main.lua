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