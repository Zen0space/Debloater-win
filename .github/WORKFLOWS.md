# GitHub Workflows

This repository contains three GitHub Actions workflows for building and testing the Windows 11 Debloater.

## Workflows

### 1. Build and Release (`.github/workflows/build.yml`)

Triggers automatically when you push a version tag (e.g., `v1.0.0`) or manually via workflow_dispatch.

**What it does:**
- Sets up Node.js and Rust
- Installs dependencies
- Builds the Tauri application
- Creates a GitHub Release
- Uploads installers as release artifacts

**How to use:**
```bash
# Tag your commit for release
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

**Artifacts:**
- NSIS installer: `.exe` file
- MSI installer: `.msi` file

### 2. CI (`.github/workflows/ci.yml`)

Runs on every push to `main` or `develop` branches, and on pull requests.

**What it does:**
- Runs TypeScript type checking
- Validates the build works

**Purpose:**
- Catch errors early
- Ensure code quality before merging

### 3. Development Build (`.github/workflows/dev-build.yml`)

Triggers automatically on push to `main` or `develop` branches, or manually via workflow_dispatch.

**What it does:**
- Builds the application
- Uploads as GitHub artifacts
- Retained for 7 days

**How to use:**
1. Go to Actions tab in GitHub
2. Select "Development Build" workflow
3. Click "Run workflow"
4. Download artifacts from the run

**Purpose:**
- Test builds without creating a release
- Share development versions with testers

## Secrets (Optional)

For code signing and enhanced security, add these secrets to your repository:

### TAURI_PRIVATE_KEY (Optional)
If you want to sign your application with a code signing certificate:

1. Generate a Tauri key pair:
   ```bash
   npm install -g @tauri-apps/cli
   tauri signer generate
   ```

2. Add the private key to GitHub Secrets:
   - Go to repository Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `TAURI_PRIVATE_KEY`
   - Value: Content of the `.key` file

3. Add the password:
   - Name: `TAURI_KEY_PASSWORD`
   - Value: The password you set when generating the key

**Note:** Without these secrets, the app will still build, just without code signing.

## Manual Workflow Trigger

To manually trigger a build:

1. Go to **Actions** tab
2. Select the workflow (Build and Release or Development Build)
3. Click **Run workflow**
4. Select the branch to run on
5. Click **Run workflow**

## Downloading Artifacts

### From Releases
1. Go to **Releases** page
2. Click on the release version
3. Download the `.exe` or `.msi` file

### From Actions
1. Go to **Actions** tab
2. Click on the workflow run
3. Scroll down to **Artifacts** section
4. Click on the artifact name to download

## Badge

Add this to your README.md:

```markdown
[![Build](https://github.com/YOUR_USERNAME/debloater-win/actions/workflows/build.yml/badge.svg)](https://github.com/YOUR_USERNAME/debloater-win/actions/workflows/build.yml)
```

Replace `YOUR_USERNAME` with your GitHub username.

## Troubleshooting

### Build Fails
- Check the Actions logs for specific errors
- Ensure all dependencies are in package.json
- Verify Rust and Node.js versions are compatible

### Release Not Created
- Verify you're pushing a tag that matches `v*.*.*` pattern
- Ensure GitHub token has write permissions
- Check if a release with the same tag already exists

### Artifacts Not Found
- Artifacts are retained for 30 days (release) or 7 days (dev build)
- Check if the workflow completed successfully
- Ensure the build generated the expected files
