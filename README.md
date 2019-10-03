# monrs
Small tool to configure current monitor layout under linux

This tool can be written easily in bash, but since I've just started learning rust I decided to give it a go.

The problem I had which brought me to this tool is that I often put my laptop on and off the dock and the monitors ports
(e.g DisplayPort-3) are renumbered, so my main monitor is attached to DisplayPort-4, secondary to DisplayPort-6.
As a result I can't use a simple bash without putting some complex logic in it, and frankly I'm not big fan of bash when
I have to deal with a lot of arguments.

### How it works

Executed with no parameters it will only print the attached monitors
```

% monrs 

Connected monitors:
0. CMN N140HCG-GQ2 (1920x1080) on eDP
1. DELL P2317H (1920x1080) on DisplayPort-2
2. HP LP2475w (1920x1200) on DisplayPort-3

Error: No valid arguments provided for current monitor setup
```


Let's say we need to position the DELL monitor on the left, rotated by 270 degrees,
the HP on center with slight Y offset of 300 px, and the laptop one on the right

```
% monrs 1:270 2::300 0
```

### Arguments

one of **idx**:***rotation***:***x***:***y***:***on*** per each monitor, where

* **idx** monitor index as printed from the tool itself when no arguments are provided

* **rotation** - optional, 0, 90, 180, 270, 0 by default

* **x** - optional, X position of the monitor relative to the left one, or 0 if it's first

* **y** - optional, Y position of the monitor, absolute, 0 by default

* **on** - optional, 1 by default, 0 to turn of the monitor 


If there isn't an argument for a monitor then that monitor will be turned off.

```
# For example to turn only the laptop display ON and to turn OFF all others:
monrs 0
```

### Requirements
You need **xrandr** and **edid-decode** and rust compiler

###Installation
Execute the following from the source directory

```
% cargo install --path . --force
``` 
