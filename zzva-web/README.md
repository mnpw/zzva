# zzva-web
> zwei zero vier acht / 2048, the game

---

Web layer of zzva. This includes various endpoints exposed via `actix-web` that allow you to interact with the game.

## Quick Start
Use `cargo r` to run the server. Uses port `8080`.

## Methods
### get
- `/health_check`
- `/view` -> Get prettified view of game board
- `/view_raw` -> -> Get raw view of game board
- `/status` -> Get status: Won, Lost, InProgress
### post
- `/start` : `"board=<board>&max=<max>"` -> Start game with chosen board size and winning tile
- `/start_default` -> Start game with default settings
- `/play` : `"direction=<direction>"` -> Play move in a direction