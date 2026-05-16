# osrs-highscores-cli

A command-line tool that fetches and displays Old School RuneScape player stats using the official OSRS Hiscores API.

Skills are displayed in a grid matching the in-game skill tab layout.

## Usage

```bash
cargo run -- --rsn <player_name>
```

## Build

```bash
cargo build --release
```

The binary will be at `target/release/osrs-highscores-cli`.

## Dependencies

- [clap](https://crates.io/crates/clap) — CLI argument parsing
- [reqwest](https://crates.io/crates/reqwest) — HTTP client (blocking)
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) — JSON deserialization


```
❯ cargo run -- --rsn ticker69lol

+------------------++------------------++------------------+
| Attack           || Hitpoints        || Mining           |
| Lv 89            || Lv 99            || Lv 85            |
| 64%              || 100%             || 3%               |
+------------------++------------------++------------------+
+------------------++------------------++------------------+
| Strength         || Agility          || Smithing         |
| Lv 97            || Lv 79            || Lv 84            |
| 28%              || 82%              || 6%               |
+------------------++------------------++------------------+
+------------------++------------------++------------------+
| Defence          || Herblore         || Fishing          |
| Lv 87            || Lv 83            || Lv 79            |
| 43%              || 26%              || 43%              |
+------------------++------------------++------------------+
+------------------++------------------++------------------+
| Ranged           || Thieving         || Cooking          |
| Lv 96            || Lv 76            || Lv 85            |
| 73%              || 6%               || 48%              |
+------------------++------------------++------------------+
+------------------++------------------++------------------+
| Prayer           || Crafting         || Firemaking       |
| Lv 78            || Lv 78            || Lv 75            |
| 45%              || 78%              || 95%              |
+------------------++------------------++------------------+
+------------------++------------------++------------------+
| Magic            || Fletching        || Woodcutting      |
| Lv 96            || Lv 80            || Lv 81            |
| 14%              || 60%              || 26%              |
+------------------++------------------++------------------+
+------------------++------------------++------------------+
| Runecraft        || Slayer           || Farming          |
| Lv 73            || Lv 91            || Lv 89            |
| 91%              || 53%              || 58%              |
+------------------++------------------++------------------+
+------------------++------------------++------------------+
| Construction     || Hunter           || Sailing          |
| Lv 83            || Lv 80            || Lv 1             |
| 29%              || 28%              || 0%               |
+------------------++------------------++------------------+

```