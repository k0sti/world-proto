# 3D Geometry Renderer

A web application that renders animated 3D geometry using Rust/WebAssembly for geometry generation and Three.js for rendering.

## Features

- High-performance geometry generation in Rust/WASM
- Real-time 3D rendering with Three.js
- Interactive camera controls (OrbitControls)
- Multiple primitive shapes (Sphere, Cube, Torus, Icosahedron)
- Animated transformations (rotation, scaling, morphing)
- Modern monorepo structure with Bun workspaces

## Tech Stack

- **Rust/WebAssembly**: Core geometry engine
- **Three.js**: 3D rendering and scene management
- **TypeScript**: Type-safe frontend development
- **Vite**: Fast build tool and dev server
- **Bun**: Package manager and workspace management

## Prerequisites

- [Rust](https://rustup.rs/) (with `wasm32-unknown-unknown` target)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Bun](https://bun.sh/)

## Installation

1. Clone the repository
2. Install dependencies:
   ```bash
   bun install
   ```

3. Build the WASM module:
   ```bash
   bun run build:wasm
   ```

## Development

Run the development server:
```bash
bun run dev
```

The application will open at http://localhost:3000

## Building for Production

Build the entire application:
```bash
bun run build
```

The production build will be in `packages/app/dist/`

## Project Structure

```
├── crates/
│   └── geometry-engine/     # Rust WASM geometry engine
├── packages/
│   ├── app/                # Frontend application
│   ├── geometry-lib/        # TypeScript wrapper for WASM
│   └── wasm/               # Generated WASM output
├── Cargo.toml              # Rust workspace configuration
└── package.json            # Bun workspace configuration
```

## Scripts

- `bun run dev` - Start development server
- `bun run build` - Build for production
- `bun run build:wasm` - Build WASM module
- `bun run clean` - Clean build outputs