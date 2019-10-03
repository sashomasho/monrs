# monrs
Small tool to easily configure multi monitor layout under linux running X.

This tool can be written in bash (and maybe someone has done it already?), but since I've just started learning rust I decided to give it a go.

The problem I had which pushed me to write this tool is that I often attach and detach my laptop to/from its dock, and as a result the monitor ports (e.g DisplayPort-3) are re-enumerated and I have to manually reposition all of my monitors  using **arandr**. Provided that the ports may differ from one attachment to another it was hard to write a bash without putting some complex logic in it, and frankly I'm not big fan of bash when I have to deal with a lot of arguments. 

Another one of my use cases is that sometimes when I'm using multi monitor setup I preffer to turn off my laptop monitor since I don't use it that much and I need an easy way to do that from the cli.

monrs works not with ports, but with monitors and position them in the order of the command line arguments, each of which descibes how the monitor should be placed relative to the others.

### How to run

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
2 |  1   |      2        |       0       |  8
0 |      |               +---------------+  0
  |      +---------------+
  +------+

% monrs 1:270 2::300 0
```


### Arguments

one of **idx**:***rotation***:***x***:***y*** per each monitor, where

* **idx** monitor index as printed from the tool itself when no arguments are provided

* **rotation** - optional, 0, 90, 180, 270, 0 by default

* **x** - optional, X position of the monitor, absolute if provided, placed on the right side of the left standing one if omitted or 0 if there is None

* **y** - optional, Y position of the monitor, absolute if provided, same as the offset of the left stating one if omitted


In the example above the full arguments, without omitting the defaults will be:
```
% monrs 1:270:0:0  2:0:1080:300  0:0:3000:300
```

If there isn't an argument for a monitor then that monitor will be turned off.

```
# For example to switch from dual monitor setup to single display, effectively turning all others OFF:
% monrs 0

```

### Requirements
You need **xrandr, edid-decode** and rust compiler to build.

### Installation
Execute the following from the source directory

```
% cargo install --path . --force
``` 
