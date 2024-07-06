# Enku
> Crack the puzzle get the word

## Usage

### Requirements
- [x] Rust toolchains (version >= 1.78.x)
- [x] WSL2 (Windows Subsystem for Linux)
  > Only need if you use Windows OS
- [x] Make (version >= 4.4.x)
- [x] Wasm Pack Cli (version >= 0.12.x)
- [x] Git (version >= 2.43.x)
- [x] Docker (version >= 24.0.x)
- [ ] Node (version >= 20.14.x)
  > Only if you want to test


### Installation

- Clone this repository

```sh
git clone https://github.com/afifurrohman-id/enku.git
```

- Go to project directory

```sh
cd enku
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
make run 
```

- Build app (debug)

```sh
make
```

- Build app (release)
```sh
make release
```

- Test (Unit Test)
```sh
make test
```
