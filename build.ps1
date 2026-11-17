#Requires -Version 5.1
$ErrorActionPreference = 'Stop'
Push-Location $PSScriptRoot
try {
    & cargo forge build --release
    if ($LASTEXITCODE -ne 0) { throw 'cargo forge build failed' }
    & python tools/check_relocations.py monster_teleport.nro
    if ($LASTEXITCODE -ne 0) { throw 'NRO relocation check failed' }
} finally { Pop-Location }
