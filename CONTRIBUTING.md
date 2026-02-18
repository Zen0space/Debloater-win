# Contributing to Windows 11 Debloater

Thank you for your interest in contributing to Windows 11 Debloater! We welcome contributions from the community.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Adding New Debloat Items](#adding-new-debloat-items)
- [Submitting Changes](#submitting-changes)
- [Reporting Issues](#reporting-issues)

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other community members

## How to Contribute

There are several ways to contribute:

1. **Report bugs** - Help us fix issues
2. **Suggest features** - Share ideas for improvements
3. **Submit pull requests** - Fix bugs or add features
4. **Improve documentation** - Help make docs clearer
5. **Share feedback** - Tell us what works and what doesn't

## Development Setup

See [QUICKSTART.md](QUICKSTART.md) for detailed setup instructions.

Quick setup:
```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/debloater-win.git
cd debloater-win

# Install dependencies
npm install

# Start development server
npm run tauri dev
```

## Adding New Debloat Items

### Step 1: Choose the Right Category

Edit the appropriate JSON file in the `data/` directory:

| File | Category | Description |
|------|----------|-------------|
| `apps.json` | Apps | Bloatware and pre-installed apps |
| `privacy.json` | Privacy | Telemetry, tracking, privacy settings |
| `services.json` | Services | Windows services to disable |
| `registry.json` | Registry | Registry modifications |
| `updates.json` | Updates | Windows Update settings |
| `system.json` | System | Performance optimizations |

### Step 2: Add the Item

Each item requires these fields:

```json
{
  "id": "unique-identifier",
  "name": "Human Readable Name",
  "description": "Clear description of what this does",
  "category": "category-name",
  "safe": true,
  "command": "PowerShell command to apply change",
  "rollbackCommand": "PowerShell command to undo change (optional)"
}
```

**Field Requirements:**

- `id`: Unique identifier (kebab-case, no spaces)
- `name`: Display name (max 50 characters)
- `description`: Clear explanation (max 150 characters)
- `category`: Must match the file name
- `safe`: `true` if safe for everyone, `false` if requires caution
- `command`: Valid PowerShell command to apply the change
- `rollbackCommand`: Optional, but recommended when possible

### Step 3: Test Locally

1. Run `npm run tauri dev`
2. Navigate to the category
3. Test the new item:
   - Verify it appears in the list
   - Test the toggle functionality
   - Apply the change
   - Verify it works as expected
   - Test rollback if provided

### Step 4: Document

Update the relevant documentation:
- If adding a new category, update README.md
- Update CHANGELOG.md if significant
- Add to appropriate preset in `presets.json` if applicable

## Development Guidelines

### Code Style

- **TypeScript**: Use strict mode, type all variables
- **React**: Functional components, hooks, avoid class components
- **Rust**: Follow Rust conventions, use `cargo fmt`
- **Tauri**: Use async/await for Tauri commands

### Testing

- Test all changes locally before submitting
- Verify PowerShell commands work on Windows 11
- Test both applying and rolling back changes
- Ensure UI looks correct in both light and dark modes

### Commit Messages

Use clear, descriptive commit messages:

```
type(scope): description

Examples:
feat(apps): add Microsoft Teams removal
fix(powershell): correct registry path syntax
docs(readme): update prerequisites section
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

## Submitting Changes

### Pull Request Process

1. **Fork the repository**
   ```bash
   # Click "Fork" on GitHub
   ```

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/debloater-win.git
   cd debloater-win
   ```

3. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make your changes**
   - Follow coding standards
   - Test thoroughly
   - Update documentation

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat(category): your description"
   ```

6. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**
   - Go to your fork on GitHub
   - Click "New Pull Request"
   - Describe your changes
   - Link to related issues

### Pull Request Checklist

Before submitting, ensure:

- [ ] Code compiles without errors
- [ ] All features work as expected
- [ ] Documentation is updated
- [ ] Commit messages are clear
- [ ] No unnecessary files are included
- [ ] PR description explains the changes

### Review Process

1. Automated tests run on your PR
2. Maintainers review your code
3. Feedback is provided (if needed)
4. Make requested changes
5. PR is merged when approved

## Reporting Issues

### Bug Reports

When reporting bugs, include:

1. **Description**: What happened
2. **Steps to reproduce**: How to trigger the bug
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happened
5. **Environment**:
   - Windows version
   - App version
   - Any error messages
6. **Screenshots**: If applicable

### Feature Requests

When suggesting features:

1. **Problem**: What problem does this solve?
2. **Solution**: How should it work?
3. **Alternatives**: What alternatives exist?
4. **Additional context**: Other relevant info

## Questions?

- Check existing [issues](https://github.com/YOUR_USERNAME/debloater-win/issues)
- Read the [documentation](README.md)
- Start a [discussion](https://github.com/YOUR_USERNAME/debloater-win/discussions)

## Recognition

All contributors will be recognized in the project's contributors list.

Thank you for contributing! 🎉
