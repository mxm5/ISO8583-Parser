@echo off

REM Set the URL of the file to download
set "url=https://github.com/HosseinAssaran/ISO8583-Parser/releases/download/v0.1.8/emv_parser.exe"

REM Set the absolute destination path for the downloaded file
set "destination=%~dp0/target/release/mv_parser.exe"

REM Download the file using bitsadmin
bitsadmin /transfer myDownloadJob /download /priority normal %url% %destination%

REM Wait for the download to complete (optional)
REM timeout /t 10 /nobreak

echo Download complete.
