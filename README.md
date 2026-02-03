# CV Tailor CLI Tool

## Problem Statement
A tool that automatically tailors a CV based on a given job profile by filtering and reordering relevant skills and experience.

### Features
- **Input**: A CV file and a job profile file.
- **Output**: A tailored CV that highlights relevant skills for the job.

## How to Run

### Prerequisites
- Rust (install via [rust-lang.org](https://www.rust-lang.org/))
- Cargo (comes with Rust)

### Setup
1. Clone this repository:
   ```bash
   git clone <your-repo-url>
   cd <repo-name>
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
