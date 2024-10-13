<h3 align="center">Yum-OSU!</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)](https://github.com/Yumshot/yum-osu)
[![GitHub Issues](https://img.shields.io/github/issues/Yumshot/yum-osu.svg)](https://github.com/Yumshot/yum-osu/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/Yumshot/yum-osu.svg)](https://github.com/Yumshot/yum-osu/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>

---

<p align="center"> Get your groove on!
    <br>
</p>

## 📝 Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Deployment](#deployment)
- [Usage](#usage)
- [Built Using](#built_using)
- [TODO](../TODO.md)
- [Contributing](../CONTRIBUTING.md)
- [Authors](#authors)
- [Acknowledgments](#acknowledgement)

## 🧐 About <a name = "about"></a>
**Yum-OSU!** is a rhythm game inspired by OSU!, designed to provide an engaging and customizable experience for rhythm game enthusiasts. Players can test their timing and precision across user-generated beatmaps, customize their gameplay with skins, and participate in ranked competitions. The game features a built-in beatmap editor for players to create and share their own tracks, emphasizing community-driven content and creative gameplay.

## 🏁 Getting Started <a name = "getting_started"></a>

These instructions will help you get **Yum-OSU!** up and running on your local machine for development and testing purposes. See [deployment](#deployment) for notes on how to install and play the game.

### Prerequisites
To develop or run **Yum-OSU!**, you’ll need Rust installed on your machine. Below are the steps to set it up:

#### Software Requirements:
1. **Rust**: Install Rust by following the instructions on the [Rust official website](https://www.rust-lang.org/tools/install).

#### Rust Dependencies:
Ensure your `Cargo.toml` file includes the following dependencies:

```toml
[dependencies]
macroquad = "0.4.13"  # Game engine for rendering and input
rand = "0.8.5"        # Random number generation
rhythms = "0.1.0"     # Core rhythm handling library
rodio = "0.19.0"      # Audio playback
aubio = "0.2.1"       # Audio analysis for beat detection
biquad = "0.4.2"      # Audio filtering
rayon = "1.10.0"      # Parallelism for performance
```

#### Installation Steps:
1. **Clone the repository** and navigate to the project directory:
   ```bash
   git clone https://github.com/Yumshot/yum-osu
   cd yum-osu
   ```

2. **Build and run the project** using Cargo:
   ```bash
   cargo run
   ```

These steps will set up your environment, and the game will automatically start once built.

## 🎈 Usage <a name="usage"></a>
As a developer, you can run the game locally using Cargo. After cloning the repository and installing the dependencies, simply run the game using:

```bash
cargo run
```

The game will launch, and you can play it while testing or modifying features.

## 🚀 Deployment <a name = "deployment"></a>
To deploy and run **Yum-OSU!** locally:
1. Ensure Rust is installed and up-to-date.
2. Clone the repository and build the game with `cargo run`.
3. The game will automatically start, allowing you to play.

## ⛏️ Built Using <a name = "built_using"></a>
- [Rust](https://www.rust-lang.org/tools/install) - A systems programming language for writing fast, reliable, and efficient code.

## ✍️ Authors <a name = "authors"></a>

- [@Yumshot](https://github.com/Yumshot) - Idea & Initial work

---

Let me know if you'd like any further changes!
