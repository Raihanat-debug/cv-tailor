# CV Tailor CLI Tool

## Problem Statement
A tool that automatically tailors a CV based on a given job profile by filtering and reordering relevant skills and experience.

### Features
- **Input**: A CV file and a job profile file.
- **Output**: A tailored CV that highlights relevant skills for the job.

The tool uses the **Clap crate** in Rust for parsing command-line arguments and reading input files.

---

## Real-Life Application

**How This Tool Can Be Used**:

The CV Tailor tool can be used by:
- **Job Seekers**: Quickly tailoring their CV for multiple job applications.
- **Recruiters**: Filtering CVs based on job profiles and matching them more effectively.
- **Resume Building Services**: Automating the process of generating tailored CVs for users, ensuring that the most relevant skills are highlighted.

---

## Running the Tool

### Prerequisites

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **Cargo**: Comes with Rust, used for building and running the tool.

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/cv-tailor.git
   cd cv-tailor
   ```

2. Run the tool:

   **Option A: Run from the Root Folder (Recommended)**
   ```powershell
   cargo run -- --cv CV-T/cv.txt --profile CV-T/profile.txt --out CV-T/tailored.txt
   ```

   **Option B: Run from inside the `CV-T` Folder**
   ```powershell
   cd CV-T
   cargo run -- --cv cv.txt --profile profile.txt --out tailored.txt
   ```

---

## Binary Releases & Cross-Compilation

This project uses **GitHub Actions** to automatically build and release binaries for multiple platforms whenever a new version tag (e.g., `v1.0.0`) is pushed.

### Supported Platforms
- **Windows** (`x86_64-pc-windows-msvc`)
- **Linux** (`x86_64-unknown-linux-musl`)
- **Linux ARM64** (`aarch64-unknown-linux-musl`)

For detailed instructions on **how to cross-compile manually** or use the release workflow, see [COMPILING.md](COMPILING.md).

### How to Download
1. Go to the [Releases](https://github.com/Raihanat-debug/cv-tailor/releases) page.
2. Download the compressed binary for your operating system.
3. Extract and run the executable from your terminal.

### Creating a New Release
To trigger a new build and release:
```bash
git tag v1.0.1
git push origin v1.0.1
```
The workflow will automatically compile the code for all supported platforms and upload the artifacts to the release page.
