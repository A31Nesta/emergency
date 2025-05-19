# Emergency

Emergency (`emg`) is an emergency backend made to solve
critical issues in Mathgeon.  
It serves the Mathgeon front-end as a static page and
generates random equations for the game.

# Building

Here are the commands needed to build Emergency.  
These commands assume that you're at this repository's
directory:

```bash
git clone https://github.com/A31Nesta/emergency
cd emergency/
```

## With Docker

```bash
docker build -t emergency . --platform=linux/amd64
```

## Native

```bash
cargo run --release
```