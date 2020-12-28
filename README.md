# ppkg

A package manager written in Rust.<br>
It will initially be a binary package manager, but I plan on making it a hybrid.

About the name: ppkg / Phate's Package Manager is a double entendre. Surely you can figure it out. ;)

If anyone wants a package added, just let me know.<br>
Issues, PRs, or even just messages on any platforms I'm on are appreciated!

Prerequisites:
- `mkdir -pv "$HOME/.ppkg/{downloads,opt}"`
- `cp config.toml $HOME/.ppkg/`

Current list of packages:
- Discord
- FileZilla
- Firefox
- Github CLI
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

Some packages supports multiple arches, in which case you get an interaction like this:

`$ ppkg -i "Firefox"` (the second line is user input)

```
This package supports 32-bit and 64-bit, please choose a version. (x32 or x64)
x64
File to download: 'firefox-84.0.1.tar.bz2'
Will be located at: '"/home/valley/.ppkg/downloads/firefox-84.0.1.tar.bz2"'
Extracting the tarball to /home/valley/.ppkg/opt/Firefox.
```

`$ ppkg list -a`

```
Name: Discord
Description: A proprietary instant messaging platform. It is HIGHLY recommended to use BetterDiscord or Powercord.
TODO: Prompt for BetterDiscord installation and install it if user agrees.
Version: 0.0.13
URL: https://discord.com/api/download?platform=linux&format=tar.gz

Name: FileZilla
Description: A FOSS FTP/SFTP/FTPS client.
Version: 3.51.0
URL (x32): https://dl4.cdn.filezilla-project.org/client/FileZilla_3.51.0_i686-linux-gnu.tar.bz2?h=l99fdW4NWUoX-YbpS-jiKg&x=1609166094
URL (x64): https://dl4.cdn.filezilla-project.org/client/FileZilla_3.51.0_x86_64-linux-gnu.tar.bz2?h=kdlGbGvcG_AQ6K645kfAJQ&x=1609166094

Name: Firefox
Description: Not the best browser, but better than some other choices for sure.
Version: 84.0.1
URL (x32): https://download-installer.cdn.mozilla.net/pub/firefox/releases/84.0.1/linux-i686/en-US/firefox-84.0.1.tar.bz2
URL (x64): https://download-installer.cdn.mozilla.net/pub/firefox/releases/84.0.1/linux-x86_64/en-US/firefox-84.0.1.tar.bz2

Name: Github CLI
Description: The official CLI for Github.
Version: 1.4.0
URL (x32): https://github.com/cli/cli/releases/download/v1.4.0/gh_1.4.0_linux_386.tar.gz
URL (x64): https://github.com/cli/cli/releases/download/v1.4.0/gh_1.4.0_linux_amd64.tar.gz

Name: Pale Moon
Description: The best web browser. NOTE: Only 64-bit is supported upstream, please compile from source if you require 32-bit.
Version: 28.17.0
URL: https://linux.palemoon.org/datastore/release/palemoon-28.17.0.linux-x86_64-gtk2.tar.xz
```

`$ ppkg list -i` (this will vary depending on what you have installed)

```
Packages installed:
- Discord
- FileZilla
- Firefox
- Pale Moon
```
