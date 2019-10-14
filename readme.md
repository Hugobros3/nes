# nes

Work-in-progress NES emulator inspired by [@javidx9](https://www.youtube.com/channel/UC-yuWVUplUJZvieEligKBkA) youtube 
series on [making an NES emulator](https://www.youtube.com/watch?v=F8kx56OZQhg&t=2069s).

The early stages of the emulator were openly inspired by [his C++ implementation](https://github.com/OneLoneCoder/olcNES).

Plan: kill two birds with one stone: get moar rust experience, and accomplish a childhood challenge.

## Status

 * Plays classic Super Mario Brothers fine, except for wonky sound.
 * Only the first square wave channel is emulated (poorly) at the moment.
 * Only supports MMC0 games
 * Passes nestest (except for illegal instructions).