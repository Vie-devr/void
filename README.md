# Void

### ** EDITOR IS UNFINISHED **

Code editor, written with Rust + Macroquad just for fun and experience.
Text editor component written from scratch specially for Void.

## Building
For Debian/Ubuntu: 
```
sudo apt install libgtk-3-dev
```

And then, for any platform: 
```
cargo run -r
```
Or launch build.sh, if you are on debian/derivative and wanna make .deb package and/or Windows executable.
Note that to build Windows executable you need to install x86_64-pc-windows-gnu target via rustup, like this: 
```
rustup target install x86_64-pc-windows-gnu
```

## Fonts
JetBrains Mono: https://www.jetbrains.com/lp/mono/
