# wrapper for rclone that remembers what goes where

TODO LIST:

- [x] add
- [ ] copy
- [x] list
- [ ] sync
- [ ] configs:
  - config is used as db
  - toml?
  - mappings are listed
  - extra variables can be set like in cargo (i.e. ignore patterns)
- [ ] systemd service / cronjob management
- [ ] rename
  - [ ] local
  - [ ] remote
- [ ] pass-through of args
  - [ ] for goclone commands (i.e. --dry-run would be passed to rclone)
  - [ ] for non goclone commands (make complete wrapper)
- [ ] [syncing of git repos](https://www.sobyte.net/post/2021-12/using-dropbox-as-git-remote-rep/)
  - how to specify different set of ignored files for backup?
    (maybe look at how branch specific git ignores work)
