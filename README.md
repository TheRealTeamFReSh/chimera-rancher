# Chimera Rancher

![CI build](https://github.com/TheRealTeamFReSh/chimera-rancher/actions/workflows/ci.yml/badge.svg)
![WASM build](https://github.com/TheRealTeamFReSh/chimera-rancher/actions/workflows/wasm.yml/badge.svg)
[![Bevy](https://img.shields.io/badge/Bevy%20Version-v0.7-blue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)


![Chimera rancher](assets/main_menu.png)

## Description

You have been an apprentice to a powerful sorcerer for as long as you can remember. However you dream of becoming a rancher, raising animals in your own pen.

The day your master died, you decided to quit the wizard tower, heading to the small town of TotallyNormalTown. This town is full of friendly people and a great variety of animals as well!

But do not forget, you are a sorcerer yourself. Everything went great until the day you split an animal in half and take the parts with you to build your own ranch of chimeras.

Now angry villagers will try to make you leave the town. Defeat them using your "friendly" chimeras and try to survive as long as possible.

Good luck.

## How to play the game

The game plays with both <kbd>W</kbd><kbd>A</kbd><kbd>S</kbd><kbd>D</kbd> and the arrow keys ⬆️⬅️⬇️➡️ for movement.

To catch an animal press <kbd>E</kbd> when in range of an animal.

To spawn a chimera with random parts press <kbd>P</kbd> and the chimera should spawn under your cursor.

In order to see the stats of an animal or a chimera, click left with the mouse 🖱️ on it.



## How to build and run

### Linux, Windows, Mac

Clone the project: 

```bash
git clone https://github.com/TheRealTeamFReSh/chimera-rancher
cd chimera-rancher
```

Run `cargo run --release` inside the project's folder:

```bash
cargo run --release
```

Enjoy!

### WASM build

Install wasm toolchain:

```bash
rustup target install wasm32-unknown-unknown
```

Build the wasm binary:

```bash
cargo build --release --no-default-features --target wasm32-unknown-unknown
```

Run bindgen on the binary:

```bash
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/chimera-rancher.wasm
```

You should have an `/out` folder containing the binary and a `.js` file.  
Now copy the web template and assets to the `/out` folder:

```bash
cp -R ./wasm-page-template/* ./out/
cp -R ./assets/ ./out/
```

Use a web server to deploy the app, e.g. python:

```bash
python -m http.server 8080 --directory ./out/
```

The last step is to visit the local website and enjoy the game: [http://localhocp -R ./assets/ ./out/cp -R ./assets/ ./out/st:8080/](http://localhost:8080/)

## About us

- [cdsupina](https://micronote.tech/) 
- tigleym
- hedgein
- [nightlyside](https://nightlyside.github.io): French student in computer science and computer security engineering
