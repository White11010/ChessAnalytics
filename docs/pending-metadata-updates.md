# Pending metadata updates (apply in Agent mode)

Plan mode could not edit non-markdown files. Apply these changes manually or re-run the agent.

## Rename license file (optional)

GitHub recognizes `LICENSE.md`. For the conventional root file name:

```bash
mv LICENSE.md LICENSE
```

Then update links in `README.md` from `LICENSE.md` to `LICENSE`.

## package.json

Add after `"version": "0.1.0",`:

```json
  "description": "Blindspot — desktop chess analytics for Lichess: sync games, Stockfish analysis, insights, and head-to-head comparison.",
  "keywords": [
    "chess",
    "lichess",
    "analytics",
    "stockfish",
    "tauri",
    "vue",
    "desktop"
  ],
  "author": "Vladislav Beliavski",
  "license": "MIT",
  "engines": {
    "node": ">=20"
  },
```

## src-tauri/Cargo.toml

```toml
description = "Blindspot — desktop chess analytics for Lichess"
authors = ["Vladislav Beliavski"]
```

## src-tauri/tauri.conf.json

- `"productName": "Blindspot"`
- window `"title": "Blindspot"`

## index.html

- `<title>Blindspot</title>`
