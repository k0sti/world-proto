# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A 3D terrain renderer using Rust/WebAssembly for procedural terrain generation and Three.js for rendering. The application features real-time terrain generation based on camera position, with separate execution threads for geometry generation and rendering.

## Essential Commands

### Development
```bash
# Start development server (builds WASM first, then starts Vite)
bun run dev

# Build WASM only
bun run build:wasm

# Build WASM in development mode (faster, unoptimized)
bun run build:wasm:dev

# Build everything for production
bun run build

# Clean all build artifacts
bun run clean
```

### WASM Package Name Issue
After running `bun run build:wasm`, the generated `packages/wasm/package.json` has `"name": "geometry-engine"` but needs to be `"@workspace/wasm"` for the workspace to resolve correctly. This must be fixed manually after the first build.

## Architecture

### Core Components

1. **Rust/WASM Geometry Engine** (`crates/geometry-engine/`)
   - `TerrainGenerator`: Generates procedural terrain using multi-octave noise
   - Takes camera position (x, z) and radius as parameters
   - Grid size is configurable (affects mesh resolution)
   - Generates vertices, indices, and normals for Three.js consumption

2. **Web Worker Architecture** (`packages/app/src/workers/`)
   - Geometry generation runs in a Web Worker (background thread)
   - Maintains ~30 FPS generation rate independent of render loop
   - Uses transferable objects for zero-copy buffer transfer
   - Camera updates are throttled (100ms) to reduce overhead

3. **Rendering Pipeline** (`packages/app/src/renderer/`)
   - `SceneManager`: Handles Three.js scene, camera, and controls
   - Uses PointerLockControls for FPS-style camera movement
   - WASD for movement, mouse for looking, Space/Shift for vertical
   - Decoupled from geometry generation - applies latest geometry when available

4. **TypeScript/WASM Bridge** (`packages/geometry-lib/`)
   - Wraps the WASM module with TypeScript interfaces
   - Handles async initialization of WASM
   - Provides type-safe access to Rust functions

### Data Flow

1. Camera moves → Main thread sends position to worker (throttled)
2. Worker generates terrain in background using WASM
3. Worker transfers geometry buffers to main thread (zero-copy)
4. Main thread applies geometry to Three.js mesh when available
5. Render loop continues at 60+ FPS regardless of generation speed

### Key Design Decisions

- **Separated Execution**: Geometry and rendering run independently for smooth performance
- **Procedural Terrain**: Uses noise functions based on world coordinates for consistent terrain
- **Grid-based Generation**: Terrain is generated as a grid centered on camera position
- **Double FPS Counters**: Separate counters for render and geometry generation performance

## Project Structure

```
├── crates/geometry-engine/       # Rust WASM engine
│   ├── src/
│   │   ├── lib.rs               # WASM bindings
│   │   └── geometry/
│   │       ├── terrain.rs       # Procedural terrain generator
│   │       ├── heightmap.rs     # Legacy animated heightmap (unused)
│   │       └── primitives.rs    # Legacy shape generators (unused)
│   └── Cargo.toml
├── packages/
│   ├── app/                     # Frontend application
│   │   ├── src/
│   │   │   ├── main.ts          # Application entry point
│   │   │   ├── workers/         # Web Worker for geometry generation
│   │   │   ├── geometry/        # Geometry controllers
│   │   │   └── renderer/        # Three.js scene management
│   │   └── vite.config.ts       # Vite config with worker support
│   ├── geometry-lib/            # TypeScript WASM wrapper
│   └── wasm/                    # Generated WASM output (gitignored)
└── package.json                 # Bun workspace root
```

## Development Workflow

When modifying terrain generation:
1. Edit `crates/geometry-engine/src/geometry/terrain.rs`
2. Run `bun run build:wasm` to rebuild
3. Fix package name in `packages/wasm/package.json` if needed
4. Restart dev server if running

When modifying rendering or controls:
1. Edit files in `packages/app/src/renderer/`
2. Changes hot-reload automatically

When modifying the worker or async architecture:
1. Edit `packages/app/src/workers/geometry-worker.ts`
2. Vite will rebuild the worker automatically
3. Worker uses ES modules and supports WASM via vite config

## Debug Panel

Located in top-right corner, shows:
- Render FPS: Main thread rendering performance
- Geometry FPS: Background generation rate
- Grid Size: Adjustable terrain mesh resolution (20-500)
- Vertices/Triangles: Current mesh statistics