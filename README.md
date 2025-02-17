# RustyMath - Scientific Calculator with Rust + WebAssembly

A modern scientific calculator built with Rust, WebAssembly, and Yew. Features a clean dark theme interface and MathLive integration for mathematical input.

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [wasm32-unknown-unknown target](https://rustwasm.github.io/docs/book/game-of-life/setup.html#rust-toolchain) (`rustup target add wasm32-unknown-unknown`)

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/sarvagyad37/rustymath-calculator.git
cd rust-calc-wasm
```

2. Install dependencies:
```bash
cargo build
```

3. Run the development server:
```bash
trunk serve
```

4. Open your browser and navigate to:
```
http://127.0.0.1:8080
```

## Development

### Project Structure
```
rustymath-calculator/
├── src/
│   ├── components/     # Yew components
│   │   ├── mod.rs
│   │   ├── calculator.rs
│   │   └── input.rs
│   └── lib.rs         # Main library file
├── index.html         # HTML template
├── Cargo.toml         # Rust dependencies
└── README.md
```

### Key Features
- MathLive integration for mathematical input
- LaTeX display for mathematical expressions

### Technologies Used
- Rust
- WebAssembly
- Yew Framework
- Tailwind CSS
- MathLive

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
- [Yew Framework](https://yew.rs/)
- [MathLive](https://cortexjs.io/mathlive/)
- [Tailwind CSS](https://tailwindcss.com/) 