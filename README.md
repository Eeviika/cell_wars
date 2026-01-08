# Cell Wars!

A strategy game played in the terminal, all about destroying the opponent's cities before they destroy yours!

## Install

Pre-releases are published on a weekly basis. These may contain bugs.

Normal releases are published... whenever the game isn't buggy. I don't really know...

You can download a release from the right side of your screen, where it says Releases.

Alternatively, you can Build It Yourself!

**Note: When running the game on Linux, make sure to launch it in a terminal. Otherwise, the game will not run (it needs a terminal to render). Windows automatically opens a terminal.**

## Building

This is actually more simple than it sounds, since rust and cargo make this easy.

Make sure you have Rust installed. If you don't, you can grab it here: https://rust-lang.org/learn/get-started/

If you already have it installed, make sure you update! (`rustup update`.)

1. Clone the repo `git clone https://github.com/Eeviika/cell_wars.git`
2. `cd` into it
3. Run `cargo build` (or `cargo build --release` for optimized builds).
4. Wait for the computer to do some math...
5. Congrats! The program should be at:
    - `./target/debug/cell_wars` (debug)
    - `./target/release/cell_wars` (release)

Essentially:
```sh
git clone https://github.com/Eeviika/cell_wars.git
cd cell_wars
cargo build
```

## FAQ

Q: Why is there no macOS release?
A: Unsigned terminal binaries are often blocked by macOS security. I don't really know why... Anyways, you're gonna have to **Build It Yourself!**

Q: When will the game be finished?
A: Probably within the next few weeks. It's not a challenging project.

Q: Can I contribute?
A: Sure! Follow the steps in **Building** to get set up. Please base your changes off the `dev` branch.

Q: The game panicked!
A: If the game crashes, it should create a log at `./panic.log`. Please attach that when making an issue.

Q: The game panicked, and it says "TODO"?
A: Most likely you're running a pre-release binary. If this happens, it's not a bug and is being worked on. If you're **not** in a pre-release binary, then please submit an issue.

Q: TEH EPIC DUCK IS COMIN!!1!1!!!
A: We gotta save Builderman! And only **you** can help us!

