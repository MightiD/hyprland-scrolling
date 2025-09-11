I liked the feature in the Cosmic desktop environment where i could use the up/down arrow keys, and a window or the focus would move up or down the workspaces associated with the monitor it was on  
i havent found a way to do this on hyprland yet, so im doing it in rust using the `hyprctl` command line tool to move windows or the focus

make the file `~/.config/hypr/scrolling.json`  
in the file you need to enter the layout you want to use 
```json
{
    "groups" : [
        [1, 4, 7],
        [2, 5, 8],
        [3, 6, 9]
    ]
}
```
each array represents a monitor you want to group  
each item in the array represents the workspace number that you want to go through, in ascending order

then add it however you want to your hyprland.conf file  
this is what i use:
```conf
# ctrl for move focus
# ctrl shift for move window
bind = $mainMod CONTROL, up, exec, ~/.local/bin/hyprland-scrolling move-focus up
bind = $mainMod CONTROL, down, exec, ~/.local/bin/hyprland-scrolling move-focus down

bind = $mainMod CONTROL SHIFT, up, exec, ~/.local/bin/hyprland-scrolling move-window up
bind = $mainMod CONTROL SHIFT, down, exec, ~/.local/bin/hyprland-scrolling move-window down
```
