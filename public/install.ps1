$ErrorActionPreference = "Stop"
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$Repo = "monkseekee-max/HMG"
$GitUrl = "https://github.com/$Repo.git"
$ReleaseBaseUrl = if ($env:HMG_RELEASE_BASE_URL) { $env:HMG_RELEASE_BASE_URL } else { "http://120.27.148.29/HMG/releases/latest/download" }
$PublicReleaseBaseUrl = if ($env:HMG_PUBLIC_RELEASE_BASE_URL) { $env:HMG_PUBLIC_RELEASE_BASE_URL } else { "https://raw.githubusercontent.com/monkseekee-max/HMG-test/main/public/releases/latest/download" }
$GitHubReleaseBaseUrl = "https://github.com/$Repo/releases/latest/download"
$BinDir = if ($env:HMG_INSTALL_DIR) {
  $env:HMG_INSTALL_DIR
} elseif ($env:LOCALAPPDATA) {
  Join-Path $env:LOCALAPPDATA "Programs\HMG\bin"
} elseif ($env:USERPROFILE) {
  Join-Path $env:USERPROFILE ".local\bin"
} else {
  Join-Path $HOME ".local\bin"
}
$TempDir = Join-Path ([IO.Path]::GetTempPath()) ("hmg-install-" + [Guid]::NewGuid().ToString("N"))

function Log([string] $Message) {
  Write-Host $Message
}

function Need-Cmd([string] $Name) {
  return [bool] (Get-Command $Name -ErrorAction SilentlyContinue)
}

function Normalize-PathEntry([string] $Entry) {
  if ([string]::IsNullOrWhiteSpace($Entry)) {
    return ""
  }

  $Expanded = [Environment]::ExpandEnvironmentVariables($Entry.Trim())
  try {
    return [IO.Path]::GetFullPath($Expanded).TrimEnd([char[]]@('\', '/'))
  } catch {
    return $Expanded.TrimEnd([char[]]@('\', '/'))
  }
}

function Path-Contains([string] $PathValue, [string] $Entry) {
  $NormalizedEntry = Normalize-PathEntry $Entry
  foreach ($PathPart in ($PathValue -split ";")) {
    if ((Normalize-PathEntry $PathPart).Equals($NormalizedEntry, [StringComparison]::OrdinalIgnoreCase)) {
      return $true
    }
  }
  return $false
}

function Add-Hmg-To-Path {
  $NormalizedBinDir = Normalize-PathEntry $BinDir
  $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")

  if (-not (Path-Contains $UserPath $NormalizedBinDir)) {
    $NewUserPath = if ([string]::IsNullOrWhiteSpace($UserPath)) {
      $NormalizedBinDir
    } else {
      $UserPath.TrimEnd([char[]]@(';')) + ";" + $NormalizedBinDir
    }
    [Environment]::SetEnvironmentVariable("Path", $NewUserPath, "User")
    Log "Added HMG bin directory to your user PATH."
  } else {
    Log "HMG bin directory is already in your user PATH."
  }

  if (-not (Path-Contains $env:Path $NormalizedBinDir)) {
    $env:Path = if ([string]::IsNullOrWhiteSpace($env:Path)) {
      $NormalizedBinDir
    } else {
      $env:Path.TrimEnd([char[]]@(';')) + ";" + $NormalizedBinDir
    }
    Log "Added HMG bin directory to this PowerShell process PATH."
  }
}

function Target-Triple {
  $Arch = if ($env:PROCESSOR_ARCHITEW6432) { $env:PROCESSOR_ARCHITEW6432 } else { $env:PROCESSOR_ARCHITECTURE }
  switch ($Arch.ToUpperInvariant()) {
    "AMD64" { return "x86_64-pc-windows-gnu" }
    "ARM64" { return "" }
    default { return "" }
  }
}

function Supported-Targets {
  Log "Supported Windows prebuilt packages:"
  Log "  hmg-x86_64-pc-windows-gnu.zip"
  Log "Windows ARM64 currently falls back to source install until an ARM64 zip is published."
}

function Download-File([string] $Url, [string] $OutputPath) {
  $LastError = $null
  for ($Attempt = 1; $Attempt -le 3; $Attempt++) {
    $Client = $null
    try {
      $Client = New-Object Net.WebClient
      $Client.DownloadFile($Url, $OutputPath)
      return
    } catch {
      $LastError = $_
      Start-Sleep -Seconds $Attempt
    } finally {
      if ($Client) {
        $Client.Dispose()
      }
    }
  }
  throw $LastError
}

function Install-From-Release-Url([string] $Asset, [string] $BaseUrl) {
  $Url = $BaseUrl.TrimEnd("/") + "/" + $Asset
  $ArchivePath = Join-Path $TempDir $Asset
  $PackageDir = Join-Path $TempDir "package"

  if (Test-Path $PackageDir) {
    Remove-Item -Recurse -Force $PackageDir
  }
  New-Item -ItemType Directory -Force $PackageDir | Out-Null

  Log "Trying HMG release: $Url"
  try {
    Download-File $Url $ArchivePath
  } catch {
    Log "Release unavailable or download failed: $Url"
    return $false
  }

  try {
    Expand-Archive -Path $ArchivePath -DestinationPath $PackageDir -Force
  } catch {
    Log "Downloaded release is not a valid zip package: $Url"
    return $false
  }

  $RequiredBins = @("hmg.exe", "hmg-server.exe", "hmg-hook-worker.exe")
  foreach ($Bin in $RequiredBins) {
    if (-not (Test-Path (Join-Path $PackageDir $Bin))) {
      Log "Release package is missing required binary: $Bin"
      return $false
    }
  }

  New-Item -ItemType Directory -Force $BinDir | Out-Null
  foreach ($Bin in $RequiredBins) {
    Copy-Item -Force (Join-Path $PackageDir $Bin) (Join-Path $BinDir $Bin)
  }
  return $true
}

function Install-From-Release {
  $Target = Target-Triple
  if (-not $Target) {
    Log "Unsupported Windows architecture for prebuilt install: $env:PROCESSOR_ARCHITECTURE"
    Supported-Targets
    return $false
  }

  $Asset = "hmg-$Target.zip"
  Log "Detected platform: Windows/$env:PROCESSOR_ARCHITECTURE -> $Target"

  foreach ($BaseUrl in @($ReleaseBaseUrl, $PublicReleaseBaseUrl, $GitHubReleaseBaseUrl)) {
    if ($BaseUrl -and (Install-From-Release-Url $Asset $BaseUrl)) {
      return $true
    }
  }

  Log "No prebuilt HMG release package found for $Target."
  Supported-Targets
  return $false
}

function Install-From-Cargo {
  if (-not (Need-Cmd "cargo")) {
    Log "Cargo/Rust toolchain not found."
    Log "Install Rust first: https://rustup.rs/"
    Log "Then rerun in PowerShell: iex ((New-Object Net.WebClient).DownloadString('http://120.27.148.29/HMG/install.ps1'))"
    exit 1
  }

  Log "No prebuilt HMG binary was found for this platform. Falling back to source install."
  Log "Source install requires Rust and access to $GitUrl."
  Log "Installing HMG from GitHub with cargo..."
  $CargoRoot = Join-Path $TempDir "cargo-root"
  $env:CARGO_NET_GIT_FETCH_WITH_CLI = "true"
  cargo install --git $GitUrl --root $CargoRoot hmg-server --bins --force
  if ($LASTEXITCODE -ne 0) {
    throw "cargo install failed with exit code $LASTEXITCODE"
  }

  New-Item -ItemType Directory -Force $BinDir | Out-Null
  foreach ($Bin in @("hmg.exe", "hmg-server.exe", "hmg-hook-worker.exe")) {
    $Source = Join-Path (Join-Path $CargoRoot "bin") $Bin
    if (Test-Path $Source) {
      Copy-Item -Force $Source (Join-Path $BinDir $Bin)
    }
  }
}

try {
  New-Item -ItemType Directory -Force $TempDir | Out-Null
  if (-not (Install-From-Release)) {
    Install-From-Cargo
  }

  Log ""
  Add-Hmg-To-Path
  Log ""
  Log "HMG installed to:"
  Log "  $BinDir"
  Log ""
  Log "If this PowerShell window still cannot find hmg, refresh this window with:"
  Log "  `$env:Path += ';$BinDir'"
  Log ""
  Log "Next steps:"
  Log "  hmg init -g"
  Log "  hmg doctor"
  Log "  codex"
  Log ""
  Log "Update later with:"
  Log "  hmg update"
} finally {
  if (Test-Path $TempDir) {
    Remove-Item -Recurse -Force $TempDir
  }
}
