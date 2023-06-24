# midi-key-interpreter
**Play pianos in games and websites with midi!**  
A rust program  that controls pianos in games by simulating keyboard controls from live midi input. Should work in most games and virtual pianos on Windows, Linux, and *maybe* Mac.  
There are 4 keyboard layouts supported:  
*   *__OnlinePiano__* for the small keyboard layout on some virtual piano websites (e.g. https://www.onlinepianist.com/virtual-piano)
*   *__FullOnlinePiano__* for the larger keyboard layout on some virtual piano websites (e.g. https://virtualpiano.net/)
*   *__GameLayout__* for pianos in games (e.g. gmod, roblox, most piano mods, etc.) *- identical keys to __FullOnlinePiano__, except it tries to get around most games ignoring simulated keyboard input, so it's slightly slower*
*   *__FullGameLayout__* for the extended keyboard layout in a few games (e.g. roblox)

(If a note is out of the range of the chosen layout it will shift it up or down by a few octaves)  
Sustain pedal functionality is also supported.
