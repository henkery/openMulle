# openMulle

openMulle is an attempt to make a framework to run all Mulle Meck games (and possibly more)

We use Bevy as the game engine core

# status
For now only the barebones for opening the cars game is implemented

# Running
The game can be run quite easily:
1. Acquire a copy of Mulle Meck car game (or any of it's localised sets)
2. copy all files on the game disc "movies" (or MOVIES) folder to the Assets folder of the projects
3. Have Nix installed
4. execute the following commands from the <root_dir>
```bash
nix-shell
cd openMulle
cargo run
```

# License
Just like Bevy we are dual-licensed under either
* MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

## Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.