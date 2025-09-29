# Diff Tool

![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Build](https://img.shields.io/badge/build-passing-brightgreen)

A command-line tool for comparing text files, implementing the Myers and Patience comparison algorithms in Rust.
---

## Features

- **Myers Diff**: Fast and optimal algorithm for computing the shortest edit script between two files.
- **Patience Diff**: Human-friendly diff algorithm, great for files with repeated lines.
- **Cross-project Utilities**: Shared utilities in `/utils` for file reading, patch writing, and more.
- **Command-line Tool**: Simple CLI with options to choose the diff algorithm.
- **FFI Support**: Call the diff algorithms from other languages.

---

## Project Structure

- `/diff` → Main executable (`diff_tool`)  
- `/myers` → Implementation of Myers diff algorithm  
- `/patience` → Implementation of Patience diff algorithm  
- `/ffi` → Rust library exposing `myers_diff`, `patience_diff`, and `free_diff` for FFI  
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
# Requirements
- Rust version 1.8+
- Windows: PowerShell 7+ for proper terminal color support