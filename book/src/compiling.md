# Compiling from source

To compile from source, you can run
```
cargo build --release
```
in the repo dir. You *can* omit `--release`, but that prevents a lot of optimisations that could be
done.

## Feature flags

There are two feature flags to set the rendering backend for iced. The default is `wgpu`, but you
might want to switch it to `tiny_skia` for debug purposes, which you can do by enabling the
`tiny_skia` feature.