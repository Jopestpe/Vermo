# Vermo
Vermo(**Ver mo**delos) é uma página para carregar e visualizar modelos 3d

### No Vermo é possível ⤵️

- ✅ Vizualizar modelos .glb

## Dependencias [Bevy](https://bevyengine.org/) (Debian)

```sh
sudo apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev
cargo install bevy
```

### Exportar Web (WASM)

* Instalar dependencias WASM Rust

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

* Criar script JS e WASM

```sh
cargo build --release --target wasm32-unknown-unknown \
&& wasm-bindgen --target web --out-dir ./web ./target/wasm32-unknown-unknown/release/vermo_bevy.wasm
```
