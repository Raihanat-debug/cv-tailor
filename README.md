# CV Tailor CLI Tool

## Motivation

The idea for this tool came from my own experience of frequently tailoring my CV for different job applications. Every time I apply for a job, I need to customize my CV, often manually changing the wording and highlighting specific skills that match the job description. This process is time-consuming and repetitive, which led me to think: “Can this process be automated?” The **CV Tailor CLI Tool** solves this problem by automating the process of tailoring a CV to a job description. 

Automating this task not only saves time but also ensures consistency in the way skills and experiences are presented.

---

## Problem Definition

**Why This Problem Was Chosen**:

Tailoring a CV for different job applications is a **time-consuming** process. Many job seekers spend hours customizing their CV to match the job requirements, even though there is a way to automate it based on the job description. The CV Tailor tool allows users to quickly tailor their CV to highlight the most relevant skills for the job.

The problem is also relevant because:
- Many job seekers do not have an efficient way to filter out irrelevant experiences and highlight skills that match the job profile.
- Manually tailoring a CV can lead to inconsistencies, and key skills may be overlooked.

---

## Why Automation?

**Why Automation Makes Sense**:

The process of tailoring a CV is repetitive and can be automated by identifying **key skills** from the job description and matching them with the skills listed in a CV. By automating this process, applicants can:
- **Save time** by automating the repetitive task of manually tailoring their CV.
- **Ensure consistency** in presenting skills and experiences.
- **Increase job application efficiency** by quickly adapting the CV to different job descriptions without error.

---

## Why Rust?

**Why I Chose Rust**:

I chose Rust for this project because of its **performance** and **memory safety** features, which make it an ideal choice for building fast and efficient command-line tools. Rust’s rich ecosystem, along with its powerful string manipulation and file I/O capabilities, make it perfect for tasks like **text parsing** and **file handling**.

---

## Approach

**How the Problem Was Solved**:

To solve the problem of tailoring CVs, I built a command-line tool that:
- Reads the **CV** and **job profile** files.
- Filters and **reorders** the CV content based on the skills and requirements listed in the job profile.
- **Outputs** the tailored CV to a new file, which emphasizes the most relevant skills for the job.

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
