# Diff Tool

![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Build](https://img.shields.io/badge/build-passing-brightgreen)

A powerful command-line tool for text file comparison, implementing the **Myers** and **Patience** diff algorithms in Rust. Ideal for developers who need a fast and accurate diff tool for source code, logs, or any text files.

---

## Features

- **Myers Diff**: Fast and optimal algorithm for computing the shortest edit script between two files.
- **Patience Diff**: Human-friendly diff algorithm, great for files with repeated lines.
- **Cross-project Utilities**: Shared utilities in `/utils` for file reading, patch writing, and more.
- **Command-line Tool**: Simple CLI with options to choose the diff algorithm.

---

## Project Structure

- `/diff` → Main executable (`diff_tool`)  
- `/myers` → Implementation of Myers diff algorithm  
- `/patience` → Implementation of Patience diff algorithm  
- `/utils` → Shared utilities used by both algorithms  

---

## Installation (Windows)

1. Clone the repository:

```powershell
git clone https://github.com/your-username/diff.git
cd diff/Installer
```

2. Run the PowerShell installation script:
```powershell
.\install.ps1
```

This will:

- Compile the project in release mode.
- Copy the executable to C:\Tools\diff_tool.
- Add the folder to your PATH (restart terminal after).
- Verify installation:

---

# Run Myers diff (default)
```powershell
diff_tool <fileA> <fileB>
```

# Run Patience diff
```powershell
diff_tool <fileA> <fileB> --patience
```
The tool generates a patch.diff file with the differences.

# Examples
### Myers diff
```diff
a
b
-c
b
+c
```
### Patience diff
```diff
a
b
+b
c
-b
```
