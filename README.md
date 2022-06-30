# zzva
> zwei zero vier acht / 2048, the game

---
## Quick Start
Use `cargo r` to initiate cli gameplay.


## Usage
Configuration for board size and winning tile is supported.\
To configure and run, use `cargo r -- --board <BOARD> --max <MAX>`.

To see logs, use `RUST_LOG=<LEVEL> cargo r -- --board <BOARD> --max <MAX>`

For help, use `cargo r -- --help`.

## Todo
### basic
- [x] game engine
- [x] testing
- [x] cli params and usability 
- [x] library polish and sanity
- [x] cli pretty gameplay 
- [x] logging

### extra
- [ ] play http endpoints
- [ ] play frontend
- [ ] scores
- [ ] save state