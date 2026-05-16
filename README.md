# HMG Site

Leptos CSR prototype for the HMG official website.

The site defaults to Chinese and includes a Chinese / English toggle in the header.

## Run Locally

```bash
env -u NO_COLOR trunk serve --address 127.0.0.1 --port 1420
```

Open <http://127.0.0.1:1420/>.

## Build

```bash
cargo check --target wasm32-unknown-unknown
env -u NO_COLOR trunk build --release
```

`env -u NO_COLOR` works around Trunk 0.21 parsing `NO_COLOR=1` as an invalid boolean.

## Install Commands

macOS / Linux:

```bash
curl -fsSL https://funcode.xin/HMG/install.sh | sh
```

Windows PowerShell:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; iex ((New-Object Net.WebClient).DownloadString('https://funcode.xin/HMG/install.ps1'))"
```

The Windows command explicitly enables TLS 1.2 before downloading the installer for Windows PowerShell 5.1 compatibility.

The website copies both `public/install.sh` and `public/install.ps1` into the published root so `https://funcode.xin/HMG/install.sh` and `https://funcode.xin/HMG/install.ps1` stay in sync.
