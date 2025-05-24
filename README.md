# Emergency

Emergency (`emg`) is Mathgeon's backend made with Axum in Rust.  
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
cargo build --release
```

# Running

To serve the Mathgeon frontend (Single Page Application),
Emergency reads the `EMG_FRONTEND_PATH` environment variable.  
If the environment vaiable is not set, it defaults to
`/opt/emg_frontend/`. The "frontend path" should contain the
`index.html` of the page.

You can run the server locally with the following command:

```bash
EMG_FRONTEND_PATH=/path/to/mathgeon/dist cargo run --release
```