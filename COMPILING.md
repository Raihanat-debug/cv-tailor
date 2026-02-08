# Cross-Compilation Guide

This guide details how to compile `cv-tailor` for Linux (x86_64 and aarch64) from a Windows machine.

## 1. Install and Configure Toolchains

You need to add the target architectures to your Rust installation.

```powershell
# Add x86_64 Linux target (standard Linux)
rustup target add x86_64-unknown-linux-musl

# Add aarch64 Linux target (ARM64, e.g., Raspberry Pi)
rustup target add aarch64-unknown-linux-musl
```

## 2. Build the Project

### Using GitHub Actions (Highest Portability)
Since local compilation on Windows requires complex toolchains (e.g. Docker + cross-linker), we recommend letting GitHub's Linux runners build the binaries for you.

1.  Push a new tag to your repository:
    ```bash
    git tag v1.0.4
    git push origin v1.0.4
    ```
2.  Wait for the "Release" action to complete in the "Actions" tab.

### Local Cross-Compilation (Requires Docker/Cross)
If you must build locally on Windows, you will need **Docker** installed.

1.  **Install Docker Desktop** and ensure it is running.
2.  Install `cross`:
    ```powershell
    cargo install cross
    ```
3.  Build for **x86_64 Linux** (musl libc):
    ```powershell
    cross build --release --target x86_64-unknown-linux-musl
    ```
4.  Build for **aarch64 Linux** (musl libc):
    ```powershell
    cross build --release --target aarch64-unknown-linux-musl
    ```

## 3. Locate Compiled Binaries

After a successful build (using `cross`), the binaries will be located here:

- **x86_64**: `target/x86_64-unknown-linux-musl/release/cv-t`
- **aarch64**: `target/aarch64-unknown-linux-musl/release/cv-t`

*(Note: On Windows, these files will not have an `.exe` extension)*

## 4. Testing the Binaries

You cannot run these binaries directly on Windows. To test them:

### Using WSL (Windows Subsystem for Linux)
1.  Open your WSL terminal (e.g., Ubuntu).
2.  Navigate to the project folder.
3.  Run the x86_64 binary:
    ```bash
    ./target/x86_64-unknown-linux-musl/release/cv-t --help
    ```

### Using QEMU (for aarch64)
To test the ARM binary on a standard PC, you need QEMU user-mode emulation installed in WSL or Linux.

## 5. Upload to GitHub Releases

If you built the binaries locally and want to upload them manually:

1.  Go to your GitHub repository.
2.  Click **Releases** > **Draft a new release**.
3.  Choose a tag (e.g., `v1.0.4`) or create a new one.
4.  Title the release (e.g., "v1.0.4 Release").
5.  Drag and drop your binary files (`cv-t` from both target folders) into the "Attach binaries" box.
    *   *Tip: Rename them to `cv-t-x86_64` and `cv-t-aarch64` before uploading so users know which is which.*
6.  Click **Publish release**.
