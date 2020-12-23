# ppkg

A package manager written in Rust.<br>
It will initially be a binary package manager, but I plan on making it a hybrid.

About the name: ppkg / Phate's Package Manager is a double entendre. Surely you can figure it out. ;)

Prerequisites:
- `mkdir -pv "$HOME/.ppkg/{downloads,opt}"`
- `cp config.toml $HOME/.ppkg/`

Current list of packages:
- Discord
- Firefox x32
- Firefox x64
- Github CLI x32
- Github CLI x64
- Pale Moon

Current list of commands:
- `--help`
- `-i`, `--install`
- `list`
  + `-a`, `--available` (list packages available to download/install)
  + `-i`, `--iinstalled`

How it works:
- The tarball is downloaded to `$HOME/.ppkg/downloads`
- The tarball is extracted to `$HOME/.ppkg/opt/$PKG_NAME`

When I figure out how I want to do it, binaries will be symlinked to `$HOME/.ppkg/bin/$BINARY`.

## Outputs

`$ ppkg -i "Pale Moon"` (the output will be similar for each package, I'll be using Pale Moon as an example)

```
File to download: 'palemoon-28.17.0.linux-x86_64-gtk2.tar.xz'
Will be located at: '"/home/valley/.ppkg/downloads/palemoon-28.17.0.linux-x86_64-gtk2.tar.xz"'
Extracting the tarball to /home/valley/.ppkg/opt/Pale Moon.
Finished extracting!
```

`$ ppkg list -a`

```
Name: Discord
Description: A proprietary instant messaging platform. It is HIGHLY recommended to use BetterDiscord or Powercord.
TODO: Prompt for BetterDiscord installation and install it if user agrees.
Version: 0.0.13
URL: https://discord.com/api/download?platform=linux&format=tar.gz

Name: Firefox x32
Description: Not the best browser, but better than some other choices for sure.
Version: 84.0.1
URL: https://download.mozilla.org/?product=firefox-latest-ssl&os=linux&lang=en-US

Name: Firefox x64
Description: Not the best browser, but better than some other choices for sure.
Version: 84.0.1
URL: https://download.mozilla.org/?product=firefox-latest-ssl&os=linux64&lang=en-US

Name: Github CLI x32
Description: The official CLI for Github.
Version: 1.4.0
URL: https://github.com/cli/cli/releases/download/v1.4.0/gh_1.4.0_linux_386.tar.gz

Name: Github CLI x64
Description: The official CLI for Github.
Version: 1.4.0
URL: https://github.com/cli/cli/releases/download/v1.4.0/gh_1.4.0_linux_amd64.tar.gz

Name: Pale Moon
Description: The best web browser. NOTE: Only 64-bit is supported upstream, please compile from source if you require 32-bit.
Version: 28.17.0
URL: https://linux.palemoon.org/datastore/release/palemoon-28.17.0.linux-x86_64-gtk2.tar.xz
```

`$ ppkg list -i` (this will vary depending on what you have installed)

```
Packages installed:
- Discord
- Github CLI x64
- Pale Moon
```
