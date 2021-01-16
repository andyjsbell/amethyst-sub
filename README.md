## Introduction

A simple roguelike game built with Amethyst and following the online book https://bfnightly.bracketproductions.com/

The idea is to create a simple Rogue game using the engine Amethyst (https://amethyst.rs/) and then to connect this up to a Substrate (https://www.substrate.io/) based chain to store items, score and other game like thingies...

The first thing to do is to build a game which will offer us a reasonable amount of game play from which we can then have a look at adding items to the Substrate based chain.

## Quickstart

- Clone the repository

```bash
git clone https://github.com/andyjsbell/amethyst-sub.git
cd amethyst-sub
```

- Build and run the project

```bash
cargo run
```

#### For Mac Users

This starter uses vulkan as a renderer by default. You'll want to change the backend to use `metal`, which can be done by opening the `Cargo.toml` file and changing

```toml
[features]
default = ["vulkan"]
```

to

```toml
[features]
default = ["metal"]
```

If using OSX and Metal you will require full XCode installed from the Appstore in order to compile metal shaders.
After install you may be required to run this command `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer` [reference gfx-rs issue](https://github.com/gfx-rs/gfx/issues/2472)

#### For Linux Users

You might need to install some dependencies. Please refer to [this section](https://github.com/amethyst/amethyst#dependencies) of the README for more details.

## Features

Things to come...