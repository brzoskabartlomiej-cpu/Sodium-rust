# Porting Sodium to Horizon Launcher (Rust)

## Goal
Port Sodium 0.7 and Sodium Extra 0.9.4 to Horizon Rust (Minecraft 26.3) in 1:1 parity.

## Steps
1. **Layout & Colors** (`src/sodium_style.rs`)
   - Search bar, page list (General, Quality, Performance, Advanced, HDR)
   - Scrolling list with tick boxes, sliders, cycling values
   - Description and performance impact panel (right)
   - Overlay tooltip for narrow screens

2. **Sodium Extra Features** (`src/sodium_extra.rs`)
   - Atmospheric fog controls (per-dimension overrides)
   - Cloud fog percentage control
   - Cylindrical fog shape (taller, matches chunk culling)
   - Hardened particle filtering

3. **Rendering Engine** (`src/rendering.rs`)
   - Chunk culling (matches Sodium 0.7 visible area)
   - Lighting updates
   - Performance impact tracking

4. **Settings Management** (`src/settings.rs`)
   - Pages: General, Quality, Performance, Advanced, HDR
   - Options: Tick boxes, sliders, cycling values
   - Disabled options (grey, struck through, tooltip reason)

5. **Horizon Integration** (`src/lib.rs`)
   - `horizon_mod_init` entry point
   - Metadata for `.hnmod` bundle
   - Compatibility with Horizon launcher (0.1.5+)

## Validation
- Unit tests in `graphics_ui.rs` (Horizon PR #5706)
- Parity tests against Sodium 0.7 numbers (640x360, 427x240)
- Page order and section jumps
- List scrolling (10 px notch, clamped)
- Dragging scrollbar thumb
- Slider tracks
- HDR gating
- Search filtering
- Undo and keyboard navigation

## Build
```bash
cargo build --release
# or
python3 tools/package.py (if Horizon tools available)
```

## License
BSD-3-Clause
