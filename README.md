# Scrib

Scrib is a terminal-style notepad: notes first, Linux workspace when you need it, with AI-assisted editing and clipboard intelligence.

## Product model

- **Note mode** — write and edit plain text in a terminal-inspired workspace.
- **AI mode** — type an instruction such as `rewrite professionally`, `shorten this`, or `fix grammar` against the current note.
- **Clipboard mode** — search and reuse recent copied text.
- **Bricks mode** — an educational Linux workspace where capabilities are added and verified one brick at a time.
- **Alpine session** — a real shell session can be attached behind the friendly UI; Scrib does not pretend the terminal is an AI chatbot.

## Safety model

AI-generated commands are proposals, not automatic authority. A command must pass Scrib's validation layer before the execution layer is allowed to run it. Failed commands are captured as feedback rather than silently chained into more commands.

## Development

```bash
cargo fmt
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run
```

The initial implementation is intentionally small. Platform-specific Alpine/PRoot integration belongs behind the terminal runtime boundary instead of being hard-coded into the note UI.
