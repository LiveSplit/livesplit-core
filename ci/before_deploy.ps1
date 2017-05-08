# This script takes care of packaging the build artifacts that will go in the
# release zipfile

$SRC_DIR = $PWD.Path
$STAGE = [System.Guid]::NewGuid().ToString()

cd capi\bind_gen
cargo run
cd ..\..

Set-Location $ENV:Temp
New-Item -Type Directory -Name $STAGE
Set-Location $STAGE

$ZIP = "$SRC_DIR\$($Env:CRATE_NAME)-$($Env:APPVEYOR_REPO_TAG_NAME)-$($Env:TARGET).zip"

# TODO Update this to package the right artifacts
Copy-Item "$SRC_DIR\capi\bindings\*" '.\' -recurse
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\livesplit_core_capi.dll" '.\livesplit_core.dll'
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\livesplit_core_capi.lib" '.\livesplit_core.lib'

7z a "$ZIP" *

Push-AppveyorArtifact "$ZIP"

Remove-Item *.* -Force
Set-Location ..
Remove-Item $STAGE
Set-Location $SRC_DIR
