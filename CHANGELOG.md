# Changelog

All notable changes to Windows 11 Debloater will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Windows 11 Debloater
- Bloatware removal for 16 pre-installed apps
- Privacy and telemetry controls (12 items)
- Windows services management (13 items)
- Registry tweaks (14 items)
- Windows Update controls (6 items)
- System optimizations (12 items)
- Three preset configurations (Minimal, Balanced, Full)
- Custom item selection with search
- Real-time progress tracking
- History and rollback support
- Modern Discord-like dark UI
- Responsive design

### Features
- Remove Xbox apps and gaming services
- Disable telemetry and advertising ID
- Disable Cortana and web search
- Control Windows Update behavior
- Optimize system performance
- Enable Game Mode and GPU scheduling
- Classic Windows context menu
- Dark mode support
- File extensions visibility
- And many more optimizations

### Technical
- Built with Tauri 2 + Rust
- React 19 + Vite + TypeScript
- Tailwind CSS + shadcn/ui
- Zustand for state management
- PowerShell command execution
- Automated GitHub Actions builds

## [1.0.0] - 2024-02-XX

### Added
- Initial stable release
- All debloating categories implemented
- Preset system with 3 configurations
- Progress tracking and history
- User-friendly Discord-like UI
- Automated build system

## Security Notes

- Application requires administrator privileges
- Executes PowerShell commands to modify Windows
- Always review changes before applying
- Create system restore point before use

## Upgrade Instructions

1. Download the latest installer from [Releases](https://github.com/YOUR_USERNAME/debloater-win/releases)
2. Uninstall the previous version (optional)
3. Run the new installer
4. Launch the application

## Contribution Guidelines

To add new debloat items:

1. Add the item to the appropriate JSON file in `data/`
2. Include: id, name, description, category, safe flag, command
3. Add rollback command if applicable
4. Test the changes locally
5. Submit a pull request
