# 7DRL
Here you'll find an entry to the [2020 7DRL game jam][jam].

## Build instructions
Main points to be aware of when building this game:

- The game interacts with the OS via libc and SDL2. Everything else is
  written in Rust, and therefore statically linked.

- Because of how Rust does things, libc will be dynamically
  linked. Might look into making this musl compatible in the future.

- SDL2 can be linked either way, but I'm going dynamic by default,
  since sdl is almost a standard, and can usually be found via a
  package manager.

### Dynamically linked
Dynamic linking is the default, as many Linux distributions ship SDL2
in their main repositories, and it's easy to bundle SDL2.dll with the
Windows builds. Ensure you have the relevant SDL2 development
libraries installed, and then simply:

```
cargo build --release
```

#### Windows note
The DLLs and .libs are bundled in the `lib/windows` directory. The
appropriate DLL will be copied into `target/release` when building.

### Statically linked
If you want to build SDL2 inside your executable, to avoid having to
mess about with dependencies or such, enable the `static-link`
feature, and build:

```
cargo build --release --features static-link
```

Note that this method requires `cmake` to be installed, as it is used
when building SDL2. See the [`rust-sdl2`][rust-sdl2] repository for
more information.

## License
This game is distributed under the terms of the [GNU GPLv3][license]
license.

[jam]: https://itch.io/jam/7drl-challenge-2020
[rust-sdl2]: https://github.com/rust-sdl2/rust-sdl2
[license]: LICENSE.md
