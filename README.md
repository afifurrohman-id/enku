# Enku
> Crack the puzzle get the word

## Usage

### Requirements
- [x] Rust toolchains (version >= 1.78.x)
- [x] WSL2 (Windows Subsystem for Linux)
  > Only need if you use Windows OS
- [x] Make (version >= 4.4.x)
- [x] Wasm Bindgen Cli (version >= 0.2.x)
- [x] Git (version >= 2.43.x)
- [x] Docker (version >= 24.0.x)


### Installation

- Clone this repository

```sh
git clone https://github.com/afifurrohman-id/enku.git
```

- Go to project directory

```sh
cd enku
```

- Make sure rustup target `wasm32-unknown-unknown` is installed
```sh
rustup target add wasm32-unknown-unknown
```

- Add sample image for local development
> put the image in `assets` directory

> image should meet following requirement:

- [x] Valid png format
- [x] 512x512 height and width
- [x] Filename is `sample.png`


### Run

- Run Server

```sh
make server
```

- Build app (debug)

```sh
make
```

- Build app (release)
```sh
make release
```
