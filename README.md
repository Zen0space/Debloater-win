# Windows 11 Debloater

[![Build](https://github.com/YOUR_USERNAME/debloater-win/actions/workflows/build.yml/badge.svg)](https://github.com/YOUR_USERNAME/debloater-win/actions/workflows/build.yml)
[![CI](https://github.com/YOUR_USERNAME/debloater-win/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/debloater-win/actions/workflows/ci.yml)

A modern, user-friendly Windows 11 debloater built with Tauri, React, and Rust. Remove bloatware, disable telemetry, and optimize your Windows 11 system with ease.

## Features

- **Bloatware Removal**: Remove pre-installed Windows Store apps and utilities
- **Privacy & Telemetry**: Disable tracking, telemetry, and privacy-invasive features
- **Service Management**: Disable unnecessary Windows services for better performance
- **Registry Tweaks**: Apply registry modifications for Windows customization
- **Update Control**: Configure Windows Update behavior and settings
- **System Optimization**: Apply performance optimizations and system tweaks
- **Preset System**: Quick presets (Minimal, Balanced, Full) for one-click debloating
- **Custom Selection**: Granular control over each item
- **Progress Tracking**: Real-time feedback during operations
- **History & Rollback**: Track changes and undo if needed

## Tech Stack

- **Frontend**: React 19 + Vite + TypeScript
- **UI Framework**: Tailwind CSS + shadcn/ui components
- **State Management**: Zustand
- **Backend**: Tauri 2 + Rust
- **PowerShell**: Command execution for Windows operations

## Prerequisites

Before building, ensure you have:

1. **Node.js** (v18 or higher) - [Download](https://nodejs.org/)
2. **Rust** (stable) - [Install](https://www.rust-lang.org/learn/get-started#installing-rust)
3. **PowerShell** (comes with Windows)
4. **Windows 11** (for building and running)

## Development

### Install Dependencies

```bash
npm install
```

### Run in Development Mode

```bash
npm run tauri dev
```

This will start both the Vite dev server and the Tauri development window.

## Building

### Build for Production

```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

### Build Output

The build process creates:
- **NSIS Installer**: `src-tauri/target/release/bundle/nsis/Windows 11 Debloater_1.0.0_x64-setup.exe`
- **MSI Installer**: `src-tauri/target/release/bundle/msi/Windows 11 Debloater_1.0.0_x64_en-US.msi`
- **Portable EXE**: `src-tauri/target/release/debloater-win.exe`

## GitHub Actions

This repository includes automated builds using GitHub Actions:

### Automatic Releases

When you push a version tag (e.g., `v1.0.0`), GitHub Actions will automatically:

1. Build the application
2. Create a GitHub Release
3. Upload the installers as release artifacts

```bash
# Create and push a version tag
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

### Development Builds

Every push to `main` or `develop` creates a development build:
- Available as GitHub Actions artifacts
- Retained for 7 days
- Great for testing without releases

### CI/CD

The workflows include:
- **Build and Release**: Creates official releases from tags
- **CI**: Runs tests and type checks on every push/PR
- **Development Build**: Creates test builds from main branches

See [`.github/WORKFLOWS.md`](.github/WORKFLOWS.md) for detailed documentation.

## Usage

1. **Select a Preset**: Choose from Minimal, Balanced, or Full presets
2. **Customize**: Navigate through categories and select/deselect items
3. **Apply Changes**: Click "Apply Selected" to execute all selected items
4. **Monitor Progress**: Watch real-time progress in the modal
5. **View History**: Track all changes made to your system

## Categories

- **Apps**: Bloatware and pre-installed apps
- **Privacy**: Telemetry, tracking, and privacy settings
- **Services**: Windows services management
- **Registry**: Registry tweaks and customization
- **Updates**: Windows Update configuration
- **System**: Performance optimizations and tweaks

## Warning

⚠️ **Use at your own risk**

Some modifications may affect system functionality:
- Always create a system restore point before applying changes
- Review selected items carefully before applying
- Keep backup of important data
- Some items marked "Use with caution" may have side effects

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - feel free to use this project for personal or commercial purposes.

## Credits

Built with:
- [Tauri](https://tauri.app/)
- [React](https://react.dev/)
- [Vite](https://vitejs.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
- [shadcn/ui](https://ui.shadcn.com/)
- [Lucide Icons](https://lucide.dev/)
