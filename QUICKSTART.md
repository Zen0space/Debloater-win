# Quick Start Guide

This guide will help you get the Windows 11 Debloater up and running quickly.

## For Users

### Download and Install

1. **Download the latest release**
   - Go to [Releases](https://github.com/YOUR_USERNAME/debloater-win/releases)
   - Download the `.exe` installer (recommended) or `.msi` installer

2. **Install**
   - Run the installer
   - Follow the setup wizard
   - Launch the application

3. **Use**
   - Select a preset or customize items
   - Click "Apply Selected"
   - Wait for completion

### Development Builds

To test the latest development version:
1. Go to [Actions](https://github.com/YOUR_USERNAME/debloater-win/actions)
2. Click on the latest "Development Build" workflow run
3. Download the artifacts at the bottom

## For Developers

### Initial Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/YOUR_USERNAME/debloater-win.git
   cd debloater-win
   ```

2. **Install Node.js**
   - Download from [nodejs.org](https://nodejs.org/) (v20 or higher recommended)
   - Or use [nvm](https://github.com/nvm-sh/nvm)

3. **Install Rust**
   ```bash
   # Windows (via rustup)
   winget install Rustlang.Rustup
   
   # Or visit https://www.rust-lang.org/tools/install
   ```

4. **Install dependencies**
   ```bash
   npm install
   ```

### Development

```bash
# Start development server
npm run tauri dev
```

This will:
- Start Vite dev server on http://localhost:1420
- Open Tauri development window
- Enable hot reload for React changes

### Building

```bash
# Build for production
npm run tauri build

# Build output location:
# src-tauri/target/release/bundle/
```

### Project Structure

```
debloater-win/
├── data/              # JSON files with debloat items
├── src/
│   ├── components/     # React components
│   ├── pages/         # Page components
│   ├── hooks/         # Custom React hooks
│   ├── store/         # Zustand state
│   └── types/        # TypeScript definitions
├── src-tauri/         # Rust backend
│   ├── src/          # Rust source code
│   └── Cargo.toml    # Rust dependencies
└── public/           # Static assets
```

## Adding New Debloat Items

1. **Edit the appropriate JSON file** in `data/`:
   - `apps.json` - Bloatware apps
   - `privacy.json` - Privacy settings
   - `services.json` - Windows services
   - `registry.json` - Registry tweaks
   - `updates.json` - Update settings
   - `system.json` - System optimizations

2. **Add a new item**:
   ```json
   {
     "id": "unique-id",
     "name": "Display Name",
     "description": "Description of what this does",
     "category": "category-name",
     "safe": true,
     "command": "PowerShell command here",
     "rollbackCommand": "Optional rollback command"
   }
   ```

3. **Test locally**:
   - Run `npm run tauri dev`
   - Navigate to the category
   - Test the item

4. **Submit changes**:
   - Create a pull request with your changes

## Common Commands

```bash
# Install dependencies
npm install

# Start development server
npm run tauri dev

# Build for production
npm run tauri build

# Run TypeScript check only
npm run build

# Preview production build
npm run preview
```

## Troubleshooting

### Build fails with Rust error
```bash
# Update Rust toolchain
rustup update stable

# Clean and rebuild
cargo clean
npm run tauri build
```

### Frontend changes not appearing
```bash
# Clear Vite cache
rm -rf node_modules/.vite

# Restart dev server
npm run tauri dev
```

### PowerShell commands not executing
- Ensure you're running on Windows
- Run application as Administrator
- Check PowerShell execution policy

## IDE Setup

### VS Code (Recommended)

Install these extensions:
1. **Tauri** - Tauri development support
2. **rust-analyzer** - Rust language support
3. **ESLint** - JavaScript/TypeScript linting
4. **Tailwind CSS IntelliSense** - Tailwind classes

Recommended VS Code settings (`.vscode/settings.json`):
```json
{
  "rust-analyzer.cargo.target": "x86_64-pc-windows-msvc",
  "typescript.tsdk": "node_modules/typescript/lib",
  "tailwindCSS.experimental.classRegex": [
    ["cva\\(([^)]*)\\)", "[\"'`]([^\"'`]*).*?[\"'`]"],
    ["cn\\(([^)]*)\\)", "(?:'|\"|`)([^']*)(?:'|\"|`)"]
  ]
}
```

### VS Code Tasks (`.vscode/tasks.json`):
```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "type": "npm",
      "script": "tauri:dev",
      "problemMatcher": [],
      "label": "Tauri Dev",
      "detail": "Run Tauri in development mode"
    },
    {
      "type": "npm",
      "script": "tauri:build",
      "problemMatcher": [],
      "label": "Tauri Build",
      "detail": "Build Tauri for production"
    }
  ]
}
```

## Resources

- [Tauri Documentation](https://tauri.app/)
- [React Documentation](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [PowerShell Documentation](https://learn.microsoft.com/en-us/powershell/)

## Support

- Open an issue on GitHub for bugs
- Check existing issues for solutions
- Read the [main README](README.md) for more details
