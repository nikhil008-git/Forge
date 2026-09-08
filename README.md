# Forge

A tiny **webhook delivery engine** in Rust.

You register a URL (**endpoint**), hand Forge a JSON payload (**event**), and Forge creates a job to POST that JSON to the URL (**delivery**).

```text
register URL  →  ingest JSON  →  pending delivery  →  worker POSTs it later
```

Layer 1 is a CLI only. Nothing is POSTed yet. State lives in `forge.json`.

Later layers (Axum, Postgres, auth, workers) stay local — they are not in this repo.

---

## What the project is about

Three objects. That’s the whole product.

| Thing | Meaning | Id prefix |
|---|---|---|
| **Endpoint** | Destination URL | `ep_` |
| **Event** | JSON to deliver | `evt_` |
| **Delivery** | The job that will POST it | `dlv_` |

Sending an event always creates a **pending** delivery. Workers (later) will pick those up and actually call the URL.

---

## How to start

Need Rust (`rustc` / `cargo`). Then from this folder:

```bash
cargo build
cargo run -- --help
```

---

## How to check it works

Use the **real id** printed by `endpoint add`. `ep_...` is a placeholder, not an id.

```bash
# 1. register a URL
cargo run -- endpoint add https://example.com/hook

# 2. copy the ep_... id it printed, then send JSON
cargo run -- event send --endpoint ep_PASTE_THE_ID '{"ok":true}'

# 3. you should see a pending delivery
cargo run -- delivery list

# 4. inspect saved state
cat forge.json
```

Also useful:

```bash
cargo run -- endpoint list
```

If send says `endpoint not found`, you used the wrong id. Run `endpoint list` and paste that `ep_` value.

Done when `delivery list` shows `Pending` and `forge.json` has all three arrays filled.

---

## Commands (layer 1)

```text
forge endpoint add <url>
forge endpoint list
forge event send --endpoint <ep_id> '<json>'
forge delivery list
```

Via cargo, prefix with `cargo run --`.

---

## Layout

```text
src/main.rs    CLI (clap)
src/lib.rs     library crate
src/model.rs   Endpoint, Event, Delivery
src/store.rs   read/write forge.json
```

`forge.json` is local state and gitignored. Delete it to start clean.
