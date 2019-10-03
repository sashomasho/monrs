# monrs
Small tool to configure current monitor layout under linux

This tool can be written in bash (and maybe some has done it already), but since I've just started learning rust I decided to give it a go.

The problem I had which brought me to this tool is that I often place my laptop on and off the dock, the monitor ports
(e.g DisplayPort-3) are re-enumerated and I had to manually readjust my multi monitor setup using **arandr**.
As a result I can't use a simple bash without putting some complex logic in it, and frankly I'm not big fan of bash when
I have to deal with a lot of arguments.

monrs works not with ports, but with monitors and position them by reading command line arguments, each of which descibes how the monitor should be placed relative to the others.

### How it run

Executed with no parameters it will only print the attached monitors
```
% monrs 

Attached monitors:
0. CMN N140HCG-GQ2 (1920x1080) on eDP
1. DELL P2317H (1920x1080) on DisplayPort-2
2. HP LP2475w (1920x1200) on DisplayPort-3


```

Let's say we need to position the DELL monitor on the left, rotated by 270 degrees,
the HP on center with slight Y offset of 300 px, and the laptop one on the right

```
    1080
  +------+      1920            1920
1 |      +---------------+---------------+  1
9 | DELL |      HP       |    LAPTOP     |  0
2 |  2   |      1        |       0       |  8
0 |      |               +---------------+  0
  |      +---------------+
  +------+

% monrs 1:270 2::300 0
```


### Arguments

one of **idx**:***rotation***:***x***:***y***:***on*** per each monitor, where

* **idx** monitor index as printed from the tool itself when no arguments are provided

* **rotation** - optional, 0, 90, 180, 270, 0 by default

* **x** - optional, X position of the monitor, absolute if provided, placed on the right side of the left standing one if omitted or 0 if there is None

* **y** - optional, Y position of the monitor, absolute if provided, same as the offset of the left stating one if omitted

* **on** - optional, 1 by default, 0 to turn of the monitor 

In the example above the full arguments, without using defaults will be:
```
% monrs 2:270:0:0:1  1:0:1080:300:1  0:0:3000:300:1
```

If there isn't an argument for a monitor then that monitor will be turned off.

```
# For example to switch from dual monitor setup to single display, effectively turning all others OFF:
% monrs 0
```

### Requirements
You need **xrandr edid-decode** and rust compiler to build.

### Installation
Execute the following from the source directory

```
% cargo install --path . --force
``` 
