# wrapper for rclone that remembers what goes where

TODO LIST:

- [x] add
- [x] copy
- [x] list
- [ ] sync
- [ ] configs:
  - [x] config is used as db
  - [x] toml?
  - [x] mappings are listed
  - [ ] extra variables can be set like in cargo:
    - [x] remote (carryover)
    - [ ] ignore patterns
    - [ ] systemd service y/n + time
    - [ ] allow multiple remotes
  - [ ] options:
    - [ ] exclude (move exclude into options)
    - [ ] editor
- [ ] systemd service / cronjob management
- [ ] rename
  - [ ] local
  - [ ] remote
- [ ] edit-config/config command
- [ ] [syncing of git repos](https://www.sobyte.net/post/2021-12/using-dropbox-as-git-remote-rep/)
  - how to specify different set of ignored files for backup?
    (maybe look at how branch specific git ignores work)
