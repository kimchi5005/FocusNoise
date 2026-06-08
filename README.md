# Focus Noise

Focus Noise is a small app for playing noise sounds designed for focus, rest, and sleep.
It started as something the author personally wanted: a simple noise player that works offline and stays out of the way.

The app generates sound inside the program, so no audio files or Docker setup are required.

## What You Can Do

- Play white noise
- Play brown noise
- Play, pause, and stop
- Change volume
- Adjust left and right balance
- Fade in and fade out
- See the current playback state
- Use a simple dark interface
- Switch between Japanese and English
- Save settings automatically

## Why Rust

Rust is a good fit for this app because it can produce a fast, standalone desktop executable with low CPU and memory usage.
The app uses native audio output and generates noise in real time instead of loading large sound files.

## Install for Development

Install Rust from:

https://www.rust-lang.org/tools/install

Then check that Rust is available:

```bash
cargo --version
```

## Run the App

```bash
cargo run --release
```

## Build the App

```bash
cargo build --release
```

The executable will be created here:

```text
target/release/focus-noise
target/release/focus-noise.exe
```

## Supported Platforms

Focus Noise is currently structured as a desktop app.

| Platform | Status | Notes |
| --- | --- | --- |
| Windows | Planned build target | GitHub Actions can build `focus-noise.exe`. |
| macOS | Planned build target | The same Rust desktop app should build on macOS. |
| Linux | Planned build target | Requires common audio/window system development packages when building. |
| Android | Future target | Possible, but needs a mobile app wrapper and mobile-specific testing. |
| iPhone / iPad | Future target | Possible in principle, but iOS packaging, signing, and App Store rules add extra work. |

The long-term goal is to keep the core app logic portable.
Noise generation, saved settings, labels, and playback state should stay separate from platform-specific packaging.

## Desktop Builds

This project includes a GitHub Actions workflow for Windows, macOS, and Linux.

1. Push this repository to GitHub.
2. Open the Actions tab.
3. Run the "Desktop builds" workflow.
4. Download the artifact for your platform.

Artifacts:

- `focus-noise-windows`
- `focus-noise-macos`
- `focus-noise-linux`

## Project Structure

```text
src/
  main.rs          App entry point
  app.rs           UI and screen behavior
  audio.rs         Audio output and noise generation
  localization.rs  Japanese and English labels
  model.rs         Shared enums and app state types
  settings.rs      Settings load/save
```

## Extending the App

The code is split so new features can be added without changing everything at once.

- Add a new noise type in `src/model.rs`
- Add its label in `src/localization.rs`
- Add its sound generation logic in `src/audio.rs`
- Add new saved options in `src/settings.rs`
- Add new controls or screens in `src/app.rs`

Good future additions could include a sleep timer, presets, pink noise, tray support, or startup behavior.

## Mobile Roadmap

Mobile support should be added after the desktop version is stable.
The recommended path is:

1. Keep noise generation and settings logic independent from the desktop UI.
2. Add automated tests for the audio state and settings logic.
3. Choose the mobile shell:
   - Android: Rust plus an Android wrapper, or a UI layer that can call the Rust core.
   - iOS: Rust core plus an iOS wrapper, with Apple signing and packaging.
4. Reuse the same app model where possible, and replace only platform-specific UI and packaging code.

## License

This project is licensed under the MIT License.
See [LICENSE](LICENSE) for details.
