# AngkorVector

<p align="center">
  <b>Modern, open-source vector illustration and graphic design software written in pure Rust.</b><br>
  Built by the ArtPixels team.
</p>

<p align="center">
  <img alt="Status: active" src="https://img.shields.io/badge/status-active-e8573f?style=flat-square">
  <img alt="Pure Rust" src="https://img.shields.io/badge/pure-Rust-b83a24?style=flat-square&logo=rust&logoColor=white">
  <img alt="Platforms" src="https://img.shields.io/badge/platforms-Windows%20%C2%B7%20macOS%20%C2%B7%20Linux%20%C2%B7%20Web-555555?style=flat-square">
  <a href="https://github.com/itbtdeveloper123-png/AngkorVector"><img alt="GitHub Repo" src="https://img.shields.io/badge/GitHub-AngkorVector-181717?style=flat-square&logo=github"></a>
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-555555?style=flat-square">
</p>

<p align="center">
  <a href="https://github.com/itbtdeveloper123-png/AngkorVector/releases"><b>Download AngkorVector</b></a> ·
  <a href="https://github.com/itbtdeveloper123-png/ArtPixels">ArtPixels Suite</a> ·
  <a href="https://github.com/itbtdeveloper123-png/AngkorPix">AngkorPix</a>
</p>

---

## Overview

**AngkorVector** is a fast, native vector graphics application developed in pure Rust with `egui` and `wgpu`. Designed for illustrators, designers, and creators who want clean performance, full precision, and reliable rendering.

---

## Key Features

- **Precision Vector Drawing**: Full Bézier pen tool, direct selection, anchor point editing, and curvature tools.
- **Pathfinder & Boolean Operations**: Unite, intersect, exclude, and divide vector shapes with ease.
- **Layers & Clipping Masks**: Comprehensive layer management, nested groups, and clipping paths.
- **Color & Gradients**: Linear, radial, and mesh gradients with color profile accuracy.
- **Text & Typography**: Rich text formatting, font layout, and path text operations.
- **High Performance**: Native GPU acceleration via wgpu, smooth panning, zooming, and rotation without lag.
- **Zero Electron / Zero Webview**: Native desktop executable with minimal memory footprint.

---

## ArtPixels Creative Suite

| App | Description | Repository |
|---|---|---|
| **AngkorPix** | Professional image editor with PSD layers, masks, filters and brush engine | [GitHub](https://github.com/itbtdeveloper123-png/AngkorPix) |
| **AngkorVector** | Vector art and illustration application | [GitHub](https://github.com/itbtdeveloper123-png/AngkorVector) |
| **ArtPixels Launcher** | Unified desktop hub to manage and launch the creative suite | [GitHub](https://github.com/itbtdeveloper123-png/ArtPixels) |

---

## Build from Source

Requirements: Stable Rust (1.85 or newer).

```sh
git clone https://github.com/itbtdeveloper123-png/AngkorVector.git
cd AngkorVector
cargo run --release -p angkorvector
```

---

## License

AngkorVector is open source and dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
Developed and maintained by **ArtPixels / IT BT**.
