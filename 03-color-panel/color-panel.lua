#!/usr/bin/lua

local color = {
    green = "-s 0.200000 -s 0.819608 -s 0.478431 -s 1",
    orange = "-s 1 -s 0.470588 -s 0 -s 1",
    red = "-s 0.878431 -s 0.105882 -s 0.141176 -s 1",
    yellow = "-s 0.964706 -s 0.827451 -s 0.176471 -s 1",
}

local function set_panel(panel, arg)
    local fmt = "xfconf-query --channel xfce4-panel -p /panels/%s/background-%s"
    os.execute(string.format(fmt, panel, arg))
end

local function set_all(arg)
    set_panel("panel-1", arg)
    set_panel("panel-2", arg)
end

local rgba = "rgba --create -t double -t double -t double -t double"
if #arg == 1 then
    set_all(string.format("%s %s", rgba, color[arg[1]]))
    set_all("style -s 1")
elseif #arg == 4 then
    local fmt = "%s -s %s -s %s -s %s -s %s"
    set_all(string.format(fmt, rgba, arg[1], arg[2], arg[3], arg[4]))
    set_all("style -s 1")
else
    set_all("style -s 0")
end
