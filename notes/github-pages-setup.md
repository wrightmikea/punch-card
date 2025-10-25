# GitHub Pages Setup Instructions

This document explains how to configure and maintain the GitHub Pages deployment for the IBM 1130 Punch Card Simulator.

## Initial Setup (One-Time)

After pushing the `docs/` folder to GitHub, configure GitHub Pages in your repository:

1. Go to your GitHub repository: `https://github.com/wrightmikea/punch-card`
2. Click **Settings** (top navigation)
3. Scroll down and click **Pages** (left sidebar)
4. Under **Source**, select:
   - **Source**: Deploy from a branch
   - **Branch**: `main`
   - **Folder**: `/docs`
5. Click **Save**

GitHub will automatically deploy the site within a few minutes.

## Live Demo URL

Once configured, your site will be available at:

**https://wrightmikea.github.io/punch-card/**

## Local Build and Deploy Workflow

The site is built locally (not on GitHub Actions) to avoid needing Rust toolchain on GitHub servers.

### Making Updates

1. **Make changes** to the source code in `crates/web/`

2. **Test locally** during development:
   ```bash
   cd crates/web
   trunk serve
   # Open http://127.0.0.1:8080 in browser
   ```

3. **Build for production** when ready to deploy:
   ```bash
   cd crates/web
   trunk build --release
   ```
   This outputs to `../../docs/` with the correct `/punch-card/` base path.

4. **Test the production build** (optional but recommended):
   ```bash
   # Create test structure matching GitHub Pages
   mkdir -p /tmp/gh-pages-test/punch-card
   cp -r docs/* /tmp/gh-pages-test/punch-card/
   cd /tmp/gh-pages-test
   python3 -m http.server 9000
   # Open http://localhost:9000/punch-card/ in browser
   ```

5. **Commit and push** the updated docs/ folder:
   ```bash
   git add docs/
   git commit -m "build: Update production build"
   git push
   ```

6. **Wait for deployment** - GitHub Pages automatically redeploys when `docs/` changes are pushed (usually 1-2 minutes).

## Build Configuration

Key configuration files for GitHub Pages deployment:

### `crates/web/Trunk.toml`
```toml
[build]
target = "index.html"
dist = "../../docs"           # Output to docs/ folder
public_url = "/punch-card/"   # Match GitHub Pages URL structure
```

### `crates/web/index.html`
- Includes favicon reference: `<link data-trunk rel="icon" href="favicon.ico">`
- Trunk processes this and outputs to docs/

### `.gitignore`
- docs/ is **NOT** ignored (intentionally tracked for deployment)
- Comment explains this is for GitHub Pages

## Troubleshooting

### Site shows blank page
- Check browser console for 404 errors
- Verify all asset URLs use `/punch-card/` base path
- Ensure `public_url = "/punch-card/"` in Trunk.toml

### Favicon not displaying
- Verify `<link data-trunk rel="icon" href="favicon.ico">` in index.html
- Check that `docs/favicon-*.ico` exists after build
- Clear browser cache

### Changes not appearing on live site
- Confirm changes are committed to `docs/` folder
- Verify push succeeded: `git push`
- Check GitHub Actions tab for deployment status
- Wait 1-2 minutes for GitHub Pages to rebuild
- Hard refresh browser (Ctrl+Shift+R or Cmd+Shift+R)

### Build fails or produces incorrect output
- Clean build: `rm -rf docs/ && trunk build --release`
- Verify Trunk version: `trunk --version` (should be 0.21.14 or later)
- Check that wasm32 target is installed: `rustup target list | grep wasm32`

## File Structure

```
punch-card/
├── docs/                          # GitHub Pages output (tracked in git)
│   ├── index.html                 # Main HTML file
│   ├── favicon-*.ico              # Favicon with hash
│   ├── styles-*.css               # Styles with hash
│   ├── punch-card-web-*.js        # WASM JavaScript bindings
│   └── punch-card-web-*_bg.wasm   # WASM binary
├── notes/                         # Documentation (moved from docs/)
│   ├── implementation.md          # Development docs
│   ├── research.txt               # Research notes
│   └── chat.txt                   # Development logs
└── crates/web/
    ├── Trunk.toml                 # Build configuration
    ├── index.html                 # Source HTML
    └── ...                        # Source code
```

## Comparison to Reference Projects

The reference projects (toggle-nixie, knob-lamps) use GitHub Actions to build on the server:

**Reference approach:**
- GitHub Actions workflow runs `npm ci && npm run build`
- Vite builds to `dist/` folder
- GitHub Actions uploads `dist/` as artifact
- GitHub deploys artifact to Pages

**Our approach (for Rust/WASM):**
- Build locally with `trunk build --release`
- Output goes to `docs/` folder
- Commit `docs/` to git
- Push to GitHub
- GitHub Pages serves `docs/` directly (no build step)

**Why different?**
- Rust toolchain is large and slow to install on GitHub Actions
- WASM builds are already optimized by Trunk
- Local build is faster and simpler for Rust projects
- Still gets live demo at GitHub Pages URL

## Updating the Build

When you modify the source code:

1. **Always rebuild** before committing:
   ```bash
   cd crates/web && trunk build --release
   ```

2. **Verify docs/ changes** with git:
   ```bash
   git status docs/
   ```

3. **Commit all changes** together:
   ```bash
   git add -A
   git commit -m "feat: Add new feature + rebuild"
   git push
   ```

## Benefits of This Approach

✅ **No GitHub Actions needed** - Simpler, no workflow configuration
✅ **Fast deploys** - Just push static files, no build step
✅ **Local testing** - Test production build before deploying
✅ **Version controlled** - docs/ changes tracked in git history
✅ **No secrets needed** - No deployment tokens or credentials
✅ **Reliable** - No build failures on GitHub servers

## Reference

- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [Trunk Documentation](https://trunkrs.dev/)
- [Yew Framework](https://yew.rs/)
