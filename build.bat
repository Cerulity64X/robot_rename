set BUILD_ROBOT_RENAME_CFG_PASSWORD=1
cargo build --release
del .\target\release\robot-rename-fpass.exe
rename .\target\release\robot-rename.exe robot-rename-fpass.exe
set BUILD_ROBOT_RENAME_CFG_PASSWORD=
cargo build --release
