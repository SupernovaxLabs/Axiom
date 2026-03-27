@echo off
setlocal
powershell -ExecutionPolicy Bypass -NoProfile -File "%~dp0install.ps1"
endlocal
