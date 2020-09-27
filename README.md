# orbiter-rs

[Rust](https://www.rust-lang.org/) bindings for
[Orbiter 2016 Space Flight Simulator](http://orbit.medphys.ucl.ac.uk/) API.

## Usage

1. [Install Orbiter 2016 Space Flight Simulator](http://orbit.medphys.ucl.ac.uk/download.html)
1. [Install Rust](https://www.rust-lang.org/tools/install)
1. Install `i686-pc-windows-msvc` target for Rust with
   ```shell
   rustup target add i686-pc-windows-msvc
   ```
1. Set the `ORBITER_DIR` or `ORBITER_SDK` environment variable.
   In PowerShell, you can do `$env:ORBITER_DIR = "<absolute_path_to_orbiter>"` for example.
    * `ORBITER_DIR`: points to the root of your Orbiter installation (where `orbiter.exe` can be found).
    * `ORBITER_SDK`: points to the `Orbitersdk` directory of your Orbiter installation.
    * __Note__: `ORBITER_SDK` always take precedence to `ORBITER_DIR`.
1. Create a new Rust project with
   ```shell
   cargo new <project_name>
   ```
1. In the `Cargo.toml`, add the following:
   ```toml
   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   orbiter = { git = "https://github.com/emgre/orbiter-rs" }
   ```
1. Add a `.cargo/config.toml` with the following content:
   ```toml
   [build]
    target = "i686-pc-windows-msvc"

    rustflags = [
        "-Ctarget-feature=+crt-static", # Static CRT
        "-Clink-args=/NODEFAULTLIB:msvcrt.lib" # Avoid conflicts generated in OrbiterSDK
    ]
   ```
1. Write your code, build it and copy the DLL in Orbiter for fun! Check the
   [module example](examples/module) for inspiration.

## License

Licensed under the MIT license. See [LICENSE.md](./LICENSE.md) for more details.

Orbiter Space Flight Simulator (not included here) is licensed under a different
[license](http://orbit.medphys.ucl.ac.uk/terms.html).

Copyright 2020 Émile Grégoire
