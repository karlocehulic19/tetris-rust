# Rust Tetris

Tetris in terminal implemented with Rust and [ratatui library](https://ratatui.rs/)

## Terminal implementation with ratatui

- Using **Text** widget to manage tetris lines with vectors

### How it works?

Project can be separated into two parts, game(actual game logic), TUI(loader of the game implemented with ratatui)
Game and TUI talk via channels and are run on separated threads. (This lowers coupling and enables other UI's to be implemented)
