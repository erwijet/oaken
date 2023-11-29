# Oaken

## Building from Source

You can build your own binary by compiling this project's source code.

1. Install [git](https://git-scm.com/download/win) and run `$ git clone https://github.com/erwijet/oaken`, or just download this repo and extract from the zip.

2. Install [nodejs](https://nodejs.org/en/download) and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (note: you may need to restart your shell to reload your `$PATH`).

3. Install dependencies and build

```sh
$ pwd
C:\path\to\oaken

$ npm i -g pnpm
$ pnpm i
$ pnpm tauri build
```

> **Note**: (windows) if you encounter an error about `link.exe` not being found, make sure to [install the msbuild MSVC C++ build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). The workaround of simply setting the rust toolchain to use GNU GCC won't work with tauri.
