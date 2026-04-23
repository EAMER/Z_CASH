# Install protoc on Windows

## Quick Install (Recommended)

### Using Chocolatey (Easiest)
```powershell
# If you have Chocolatey installed:
choco install protoc

# Verify installation
protoc --version
```

### Using Windows Package Manager
```powershell
winget install protobuf

# Verify installation
protoc --version
```

---

## Manual Installation

### Step 1: Download
Go to: https://github.com/protocolbuffers/protobuf/releases

Download the latest **protoc-X.XX.X-win64.zip** (or win32.zip for 32-bit)

### Step 2: Extract
```powershell
# Extract to C:\protoc
Expand-Archive -Path protoc-22.0-win64.zip -DestinationPath C:\protoc
```

### Step 3: Add to PATH
```powershell
# Option A: Temporary (this session only)
$env:Path += ";C:\protoc\bin"
$env:PROTOC = "C:\protoc\bin\protoc.exe"

# Option B: Permanent (recommended)
[Environment]::SetEnvironmentVariable("Path", "$env:Path;C:\protoc\bin", "User")
[Environment]::SetEnvironmentVariable("PROTOC", "C:\protoc\bin\protoc.exe", "User")
```

### Step 4: Verify
```powershell
protoc --version
```

Should output something like: `libprotoc X.XX.X`

---

## Using Docker (Alternative)

If you prefer containerized build:

```dockerfile
FROM rust:latest

RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
```

---

## Build After Installing protoc

```powershell
cd C:\Users\USER\ZCASH_APP
cargo build --release
```

Expected output:
```
   Compiling cts-gate v0.1.0 (C:\Users\USER\ZCASH_APP)
    Finished release [optimized] target(s) in XX.XXs
```

---

## Verify Build Success

```powershell
ls target/release/cts-gate.exe
```

Should show the executable exists.

---

## Run Tests

```powershell
cargo run --release -- run --profile lightwalletd --suite mvp --output reports
```
