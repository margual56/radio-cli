![GitHub](https://img.shields.io/github/license/margual56/radio-cli) [![Rust](https://github.com/margual56/radio-cli/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/margual56/radio-cli/actions/workflows/rust.yml)


# radio-cli
A simple radio CLI written in rust

[![asciicast](https://asciinema.org/a/Kt0CP53YO0IWPyUs1p2S45zO7.svg)](https://asciinema.org/a/Kt0CP53YO0IWPyUs1p2S45zO7)

### Warning! (optional dependency)
To play youtube music you need to have `youtube-dl` installed! 

## Contributing and code of conduct
Please, take a look at the [Contributing](https://github.com/margual56/radio-cli/blob/main/CONTRIBUTING.md) and [Code of Conduct](https://github.com/margual56/radio-cli/blob/main/CODE_OF_CONDUCT.md) guidelines

## Usage
To use it, just type `radio-cli` after installing it and the program will guide you.

When playing music, __you can use the mpv keybindings__ to control it (spacebar to play/pause, etc).

# Installation
- On Arch (and derivatives such as Manjaro), you can just install it through [the AUR package](https://aur.archlinux.org/cgit/aur.git/tree/PKGBUILD?h=radio-cli-bin) called **radio-cli-bin**. If you have an AUR helper:
   ```bash
   $ yay -S radio-cli-bin
   ```
   _Note: radio-cli-git is now unsupported_<br/><br/>

- On other systems you can:
   - Install it through cargo: 
   
      `cargo install --git https://github.com/margual56/radio-cli`<br/><br/>

   - Install it manually, without automatic update capabilities:
      ```bash
      git clone https://github.com/margual56/radio-cli.git radio-cli
      cd radio-cli
      cargo build --release
      sudo cp "./target/release/radio-cli" "/usr/bin/radio"
      mkdir -p "${XDG_CONFIG_HOME}/radio-cli/"
      cp "./config.json" "${XDG_CONFIG_HOME}/radio-cli/"
      ```
   

<details>
<summary><h2>How it works...</h2></summary>
...is very simple. The idea is to have a compilation of radio stations in <a href="https://github.com/margual56/radio-cli/blob/main/config.json">the config file</a> and have a tool to be able to easily select one or the other.

The rest is thanks to the <b>wonderful</b> and <b>amazing</b> <a href="https://github.com/mpv-player/mpv">mpv player</a>. mpv is the one that does all the heavy-lifting and plays whatever you throw at it.

Let's say this is just a cli frontend for playing things on mpv 😄, kinda like <a href="https://github.com/pystardust/ani-cli">ani-cli</a> but without search functionalities and focused on radio stations.
</details>

<details>
<summary><h2>Configurability</h2></summary>
As said before, this app is just a compilation of radios. It can be found in <a href="https://github.com/margual56/radio-cli/blob/main/config.json">the config file</a> as a JSON, with a list of station names and their URLs.

Of course you can add literally WHATEVER you want, even youtube videos (again, all thanks to mpv).
</details>

## Fork me!
If you (wrongfully xD) think mpv is not the best player, go ahead, fork me and change it :)

The license is GPLv2

## Planned features 
Don't be surprised if these are not implemented in the end hehe (if there is no interest in the project, certainly not)

- [x] ~Audio (mpv) controls when not in verbose mode~
- [ ] Loop to selection list when pressing `q` while playing
- [x] ~Some kind of online updating of the list of stations~ _(kind of)_
- [x] ~Code optimizations/beautification~
- Languages:
  - [x] ~English~
  - [ ] Spanish
  - [ ] Others(?)
- [x] ~An AUR installer~
