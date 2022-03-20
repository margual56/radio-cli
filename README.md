# radio-cli
A simple radio CLI written in bash

[![asciicast](https://asciinema.org/a/ZRXVjIsGUWj6g7DyeR2V50Ge3.svg)](https://asciinema.org/a/ZRXVjIsGUWj6g7DyeR2V50Ge3?t=6)

## How it works...
...is very simple. The idea is to have a compilation of radio stations in [the config file](https://github.com/margual56/radio-cli/blob/main/config.json) and have a tool to be able to easily select one or the other.

The rest is thanks to the **wonderful** and **amazing** [mpv player](https://github.com/mpv-player/mpv). mpv is the one that does all the heavy-lifting and plays whatever you throw at it.

Let's say this is just a cli frontend for playing things on mpv 😄, kinda like [ani-cli](https://github.com/pystardust/ani-cli) but without search functionalities and focused on radio stations.

## Configurability
As said before, this app is just a compilation of radios. It can be found in [the config file](https://github.com/margual56/radio-cli/blob/main/config.json) as a JSON, with a list of station names and their URLs.

Of course you can add literally WHATEVER you want, even youtube videos (again, all thanks to mpv).

## Fork me!
If you (wrongfully xD) think mpv is not the best player, go ahead, fork me and change it :)

The license is GPLv2

## Planned features 
Don't be surprised if these are not implemented in the end hehe (if there is no interest in the project, certainly not)

- [ ] Some kind of online updating of the list of stations
- [ ] Code optimizations/beautification
- [ ] An AUR installer
