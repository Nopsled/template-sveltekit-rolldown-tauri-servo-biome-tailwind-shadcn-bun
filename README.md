# SvelteKit × Rolldown × Tauri × Servo × Biome × Tailwind × ShadCn × Bun Template

A cutting-edge desktop application template that combines the latest web technologies with Rust-powered desktop capabilities and experimental web rendering.

## 🚀 What This Template Provides

This template demonstrates a modern approach to desktop application development by combining:

- **SvelteKit v5** - The latest version of the powerful full-stack web framework
- **Rolldown via Vite** - Ultra-fast Rust-powered bundler as a Vite plugin replacement
- **Tauri v2** - Lightweight Rust-based framework for building desktop apps
- **Servo Rendering Engine** - Experimental high-performance web engine written in Rust
- **Biome v2** - Lightning-fast formatter and linter that replaces ESLint + Prettier
- **Tailwind CSS v4** - Latest version with improved performance and new features
- **ShadCN Svelte** - Beautiful, accessible UI components built with Tailwind

## ✨ Key Features

### Performance & Speed
- **Rolldown bundling** - Significantly faster builds compared to traditional bundlers
- **Servo rendering** - Experimental web engine with potential performance benefits
- **Tauri v2** - Smaller bundle sizes and better performance than Electron
- **Biome** - 10-100x faster than ESLint/Prettier combinations

### Developer Experience
- **TypeScript** support out of the box
- **Hot module reloading** for rapid development
- **Consistent code formatting** with Biome
- **Modern CSS** with Tailwind v4 features
- **Component library** with ShadCN Svelte components

### Cross-Platform
- Build for **Windows**, **macOS**, and **Linux**
- Native system integration through Tauri
- Web-based UI with native performance

## 📋 Prerequisites

Make sure you have the following installed:

- **[Bun](https://bun.sh/)** (latest version) - Package manager and runtime
- **[Rust](https://rustup.rs/)** (latest stable) - Required for Tauri
- **[Node.js](https://nodejs.org/)** (v18+) - Fallback runtime support

### Platform-Specific Requirements

#### Windows
- Microsoft C++ Build Tools or Visual Studio with C++ support
- WebView2 (usually pre-installed on Windows 10/11)

#### macOS
- Xcode Command Line Tools: `xcode-select --install`

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

## 🛠️ Setup

1. **Clone or use this template**
   ```bash
   git clone <your-repo-url>
   cd template-sveltekit-rolldown-tauri-servo-biome-tailwind-shadcn
   ```

2. **Install dependencies**
   ```bash
   bun install
   ```

3. **Install Tauri CLI** (if not already installed)
   ```bash
   bun add -g @tauri-apps/cli@latest
   ```

## 🏃‍♂️ Running the Project

### Development Mode
Start the development server with hot reloading:
```bash
bun run dev
```
This will:
- Start the SvelteKit dev server on `http://localhost:5173`
- Launch the Tauri desktop application
- Enable hot module reloading for both frontend and backend changes

### SvelteKit Only (Web Development)
If you want to develop just the web portion:
```bash
bun run sveltekit:dev
```

### Building for Production
Build the complete desktop application:
```bash
bun run build
```

### Preview Production Build
Preview the built SvelteKit application:
```bash
bun run preview
```

## 📁 Project Structure

```
├── src/                          # SvelteKit application source
│   ├── lib/                      # Shared components and utilities
│   │   ├── components/           # Reusable Svelte components
│   │   │   └── ui/              # ShadCN UI components
│   │   ├── hooks/               # SvelteKit hooks
│   │   └── utils.ts             # Utility functions
│   ├── routes/                   # SvelteKit routes (pages)
│   │   ├── +layout.svelte       # Root layout component
│   │   ├── +layout.ts           # Layout load function
│   │   └── +page.svelte         # Home page
│   ├── app.css                  # Global styles (Tailwind imports)
│   ├── app.d.ts                 # TypeScript declarations
│   └── app.html                 # HTML template
├── src-tauri/                    # Tauri Rust backend
│   ├── src/                     # Rust source code
│   ├── Cargo.toml              # Rust dependencies
│   ├── tauri.conf.json         # Tauri configuration
│   └── build.rs               # Build script
├── static/                      # Static assets
├── biome.json                  # Biome configuration
├── components.json             # ShadCN component configuration
├── package.json               # Node.js dependencies and scripts
├── svelte.config.js           # SvelteKit configuration
├── tsconfig.json             # TypeScript configuration
└── vite.config.ts           # Vite configuration (with Rolldown)
```

## 🔧 Available Scripts

| Script | Description |
|--------|-------------|
| `bun run dev` | Start development server with Tauri app |
| `bun run build` | Build production desktop application |
| `bun run sveltekit:dev` | Start only SvelteKit dev server |
| `bun run sveltekit:build` | Build only SvelteKit application |
| `bun run preview` | Preview production SvelteKit build |
| `bun run check` | Run SvelteKit type checking |
| `bun run check:watch` | Run type checking in watch mode |
| `bun run tauri` | Access Tauri CLI commands |

## 🎨 Styling & Components

### Tailwind CSS v4
This template uses Tailwind CSS v4 with:
- Modern configuration
- JIT compilation
- Custom design tokens
- Responsive design utilities

### ShadCN Svelte Components
Pre-configured with ShadCN Svelte for beautiful, accessible components:
- Button components included
- Easy to add more components via CLI
- Customizable design system
- TypeScript support

To add new ShadCN components:
```bash
npx shadcn-svelte@latest add button  # Example
```

## 🧹 Code Quality

### Biome Configuration
Biome is configured with:
- **Formatting**: 2-space indentation, single quotes, trailing commas
- **Linting**: Recommended rules enabled
- **Performance**: Much faster than ESLint + Prettier

Run code quality checks:
```bash
bunx biome check .          # Check all files
bunx biome format .         # Format all files
bunx biome lint .           # Lint all files
bunx biome check . --fix    # Fix auto-fixable issues
```

## ⚠️ Experimental Features

### Servo Rendering Engine
This template uses the experimental Servo rendering engine through `tauri-runtime-verso`. This provides:
- **Potential performance benefits** over traditional webviews
- **Modern web standards** support
- **Rust-based rendering** for consistency with Tauri

**Note**: Servo integration is experimental and may have compatibility issues. For production applications, consider using the standard Tauri webview until Servo reaches stability.

### Rolldown Bundler
Rolldown is used via `rolldown-vite` package, providing:
- **Faster builds** than traditional bundlers
- **Rust-powered performance**
- **Vite compatibility** for familiar development experience

## 🚀 Deployment

### Building for Different Platforms
```bash
# Build for current platform
bun run build

# Build for specific platforms (configure in tauri.conf.json)
bun run tauri build --target x86_64-pc-windows-msvc    # Windows
bun run tauri build --target x86_64-apple-darwin       # macOS Intel
bun run tauri build --target aarch64-apple-darwin      # macOS Apple Silicon
bun run tauri build --target x86_64-unknown-linux-gnu  # Linux
```

### Distribution
Built applications will be in `src-tauri/target/release/bundle/`:
- **Windows**: `.msi` installer and `.exe` executable
- **macOS**: `.dmg` disk image and `.app` bundle
- **Linux**: `.deb` package and `.appimage`

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run quality checks: `bunx biome check . --fix`
5. Test your changes: `bun run dev`
6. Commit your changes: `git commit -m 'Add amazing feature'`
7. Push to the branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## 📝 License

This template is available under the MIT License. See the LICENSE file for more details.

## 🔗 Useful Links

- [SvelteKit Documentation](https://kit.svelte.dev/)
- [Tauri Documentation](https://tauri.app/)
- [Servo Project](https://servo.org/)
- [Rolldown](https://rolldown.rs/)
- [Biome](https://biomejs.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
- [ShadCN Svelte](https://shadcn-svelte.com/)

## 💡 Tips

- Use `bun run dev` for the full development experience
- The Servo rendering engine is experimental - report issues to help improve it
- Biome is much faster than ESLint/Prettier - leverage it for better DX
- Take advantage of Tailwind v4's new features for better styling
- Use ShadCN components for consistent, accessible UI elements

---

**Happy coding!** 🎉 This template represents the cutting edge of desktop application development with web technologies.