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
