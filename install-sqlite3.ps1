Write-Output "Installing SQLite 3 library..."

[void](New-Item ".\tmp" -ItemType Directory -Force)

# Download source code of SQLite 3
Invoke-WebRequest -Uri "https://www.sqlite.org/2021/sqlite-amalgamation-3340100.zip" -OutFile ".\tmp\sqlite-src.zip"
Expand-Archive -Path ".\tmp\sqlite-src.zip" -DestinationPath ".\sqlite-win64"

# Download precompiled binaries of SQLite 3
Invoke-WebRequest -Uri "https://www.sqlite.org/2021/sqlite-dll-win64-x64-3340100.zip" -OutFile ".\tmp\sqlite-win64.zip"
Expand-Archive -Path ".\tmp\sqlite-win64.zip" -DestinationPath ".\sqlite-win64"

# Generate sqlite3.lib
LIB.exe /DEF:.\sqlite-win64\sqlite3.def /OUT:.\sqlite-win64\sqlite3.lib /MACHINE:X64 /NOLOGO

Write-Output "Complete!"
