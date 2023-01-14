# GOCLONE: a wrapper for rclone that remembers what goes where

[![LOC](https://tokei.rs/b1/github/probably-neb/goclone?category=lines)](https://github.com/probably-neb/goclone)

## WIP

CRITICAL:

- [ ] pass through of rclone output
  - [ ] add -P rclone progress
- [ ] improve error handling => move everything to results
- [ ] move paths from String/&str to PathBuf/&Path
- [ ] checking of config file on open / command
  - [ ] normalize paths
  - [ ] check remotes are real
  - [ ] check paths exist

TODO LIST:

- [x] add
- [x] copy
- [x] list
- [ ] sync
  - [ ] "rclone passthrough" trait or similar for
        commands that pass their input through to rclone
        and allow normal flags
- [ ] configs:
  - [x] toml config is used instead of db with mappings listed
  - [ ] extra variables (in mappings) can be set like in cargo:
    - [x] remote (carryover)
    - [x] ignore patterns
    - [ ] systemd service y/n + time
    - [ ] allow multiple remotes
    - [ ] custom flags (like platform specific)
  - [ ] options:
    - [ ] exclude (move exclude into options)
    - [ ] remote specific commands
- [ ] systemd service / cronjob management
  - [ ] how to organize?
    - one per unique time step?
    - one per item
- [ ] edit config command
- [ ] [syncing of git repos](https://www.sobyte.net/post/2021-12/using-dropbox-as-git-remote-rep/)
  - basically just creating a bare repo locally and
    syncing that with the remote on push
