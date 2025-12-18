
# Release Automation

## R.R.1: GitHub Actions release workflow

The project must use GitHub Actions to automatically build and publish binaries when a GitHub release is published (not when a draft is created). The workflow must be triggered on release publication events.

## R.R.2: Linux x64 binary build

The release workflow must build a binary for Linux x64 architecture. The binary must be compatible with modern Linux distributions and must be included as an asset in the GitHub release.

## R.R.3: Linux ARM binary build

The release workflow must build a binary for Linux ARM architecture. The binary must be compatible with modern Linux distributions and must be included as an asset in the GitHub release.

## R.R.4: macOS Intel binary build

The release workflow must build a binary for macOS Intel (x86_64) architecture. The binary must be compatible with macOS 10.15+ and must be included as an asset in the GitHub release.

## R.R.5: macOS Apple Silicon binary build

The release workflow must build a binary for macOS Apple Silicon (ARM64) architecture. The binary must be compatible with macOS 11+ and must be included as an asset in the GitHub release.

## R.R.6: Windows x64 binary build

The release workflow must build a binary for Windows x64 architecture. The binary must be compatible with Windows 10 and later, and must be included as an asset in the GitHub release.

## R.R.7: Windows ARM binary build

The release workflow must build a binary for Windows ARM architecture. The binary must be compatible with Windows 10 and later, and must be included as an asset in the GitHub release.

## R.R.8: Binary naming convention

All binaries must be named "reqlix" (without file extension) for Unix-like systems (Linux, macOS). Windows binaries must be named "reqlix.exe". The platform-specific archive files must include the binary name and platform identifier in their filenames.

## R.R.9: Archive format

All binaries must be packaged in ZIP format for distribution. Each platform-specific binary must be included in a separate ZIP archive file attached to the GitHub release.

## R.R.10: Matrix strategy for parallel builds

The GitHub Actions workflow must use a matrix strategy to build binaries for all target platforms in parallel. Each matrix entry must specify the target platform, Rust target triple, and binary name (reqlix or reqlix.exe).

## R.R.11: Rust target triples

The workflow must use the following Rust target triples: x86_64-unknown-linux-gnu (Linux x64), aarch64-unknown-linux-gnu (Linux ARM), x86_64-apple-darwin (macOS Intel), aarch64-apple-darwin (macOS Apple Silicon), x86_64-pc-windows-msvc (Windows x64), aarch64-pc-windows-msvc (Windows ARM). The workflow must install the required Rust targets using rustup target add before building.

## R.R.12: Build process

For each target platform, the workflow must: 1) Install Rust toolchain, 2) Add the required target using rustup, 3) Build the release binary using cargo build --release --target <target-triple>, 4) Locate the built binary in target/<target-triple>/release/, 5) Rename the binary to the correct name (reqlix or reqlix.exe) according to R.R.8.

## R.R.13: ZIP archive creation

After building each binary, the workflow must create a ZIP archive containing the renamed binary. The ZIP filename must follow the pattern: reqlix-<version>-<platform-identifier>.zip, where platform-identifier clearly identifies the target platform (e.g., linux-x64, linux-arm64, macos-x64, macos-arm64, windows-x64, windows-arm64). The ZIP must contain only the binary file at the root level.

## R.R.14: Artifact upload to release

After all binaries are built and packaged, the workflow must upload all ZIP archives as release assets to the published GitHub release. The workflow must use the GitHub API or GitHub CLI (gh) to attach the artifacts to the release. All artifacts must be uploaded before the workflow completes.

## R.R.15: Workflow file location

The GitHub Actions workflow must be defined in a file located at .github/workflows/release.yml. The workflow must be triggered on the release.published event type.
