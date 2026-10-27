# LisekMovie-plugin-anisearch

aniSearch metadata provider extension for [LisekMovie](https://github.com/LisekMovieApp/LisekMovie-app), the self-hosted video-on-demand server.

| | |
|---|---|
| Provider key | `anisearch` |
| Covers | Anime |
| Status | Skeleton — registered, returns no results yet |
| API key | Optional (reserved for a partner key) |
| Fix match ids | `anisearch:2227`, `https://www.anisearch.com/anime/2227,one-piece` |

## What it does

Registered skeleton. aniSearch has no public API; every method returns nothing and logs one warning per process.

**TODO:** implement the endpoints once a partner API key (or a permitted way of reading the pages) exists.

## How it plugs in

The crate implements `lisek_plugin_api::MetadataProvider` and is compiled into `lisek-server` as a Cargo path dependency — there is no runtime plugin loading. Clone it **next to** `LisekMovie-app`:

```
LisekMovie/
  LisekMovie-app/
  LisekMovie-plugin-anisearch/
  ...
```

The server lists the provider under **Admin → Plugins** (disabled by default). Turn it on, add the key if it needs one, and set its priority — globally, or per library (for example AniList or aniDB first in an anime library). `install-linux.sh` and `update-linux.sh` clone and update it with the other modules.

## Development

```bash
cargo clippy --all-targets -- -D warnings
cargo test
```

Tests run against a local `wiremock` server; nothing calls the real service.

## License

[PolyForm Noncommercial 1.0.0](https://github.com/LisekMovieApp/LisekMovie-app/blob/main/LICENSE), as LisekMovie itself.
