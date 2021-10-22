# Multisnake

A choatic multiplayer snake game where you shoot down your enemies.

## Setup and contributions

Obviously I would be happy for you too add to this game.
Here is a brief set of steps to setting up the enviorment.
Please do not distrubute self compiled standalone versions of the game without explicit permission from the maintainers, or a big enough change with credit. Do not also 

### Notes

This game makes use of Rust and Bevy, so make sure you have rustc preinstalled on your working device and ready to use. [Check out the bevy book as well](https://bevyengine.org/learn/book/introduction/).
Now bevy takes about an 5 minutes to compile normally and thats too slow, so we have the faster way to compile which is more indepth [here](https://bevyengine.org/learn/book/getting-started/setup/"), but here's a brief tutorial.

Download the LLD linker which is faster way faster then the rust linker.
Ubuntu: `sudo apt-get install lld`, Arch: `sudo pacman -S lld`, Windows: get the latest version of bin utils `cargo install -f cargo-binutils && rustup component add llvm-tools-preview`
Since the other files that are needed in the repo, that should be all you need. 

1. Open the folder system in vscode (or other editors) and perhaps let other rust/cargo plugins recognize.
2. Run cargo run, cargo build for it to compile all the modules. This may take a while like 5 minutes upwards but after that you get near 7 seconds or lower compiles so yay!
3. After it builds well, you should be ready to work on it. Woo!
