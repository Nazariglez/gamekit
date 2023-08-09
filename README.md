# GameKit

This is GameKit, a [Rust](https://rust-lang.com) Game Framework I am developing.

Please note that this framework is highly experimental and undergoes frequent breaking changes. 
It's primarily designed to meet my personal needs, rather than general use.

While I don't plan to provide active support, I aim to publish games using this framework and ensure compatibility 
with various platforms. I've organized these platforms into `tiers` based on where I intend to publish my games, 
primarily on platforms such as [Steam](https://store.steampowered.com/) and [Itch.io](https://itch.io).

* Tier 1: Windows, MacOS, Web
* Tier 2: Linux (Ubuntu)
* Tier 3: Android, iOS

## Profiling
The crate `puffin` is used to profile the application. 
You need to install [puffin_viewer](https://github.com/EmbarkStudios/puffin/tree/main/puffin_viewer).

Then run your application with feature `puffin` and use `puffin_viewer --url 127.0.0.1:8585` to see the result.

## notes
- If possible, move to notan everything that's is core and consume notas as dependency on gamekit
- use fluent-rs for translations?
- use pollster to block futures
- allow to bind multiple binding groups