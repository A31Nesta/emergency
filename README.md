# Emergency

Emergency (`emg`) is a tiny backend made with Axum in Rust.  
It serves SPAs and nothing else.

This is a stripped-down version of the backend used for
Mathgeon, a math-based web game. For the original Mathgeon
backend, see the [`mathgeon` branch](https://github.com/A31Nesta/emergency/tree/mathgeon)

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

To serve a frontend (Single Page Application),
Emergency reads the `EMG_FRONTEND_PATH` environment variable.  
If the environment vaiable is not set, it defaults to
`/opt/emg_frontend/`. The "frontend path" should contain the
`index.html` of the page.

You can run the server locally with the following command:

```bash
EMG_FRONTEND_PATH=/path/to/dist cargo run --release
```