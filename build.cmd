@echo off
REM ============================================================
REM Vortex — Build Windows
REM Génère l'installateur et le portable .exe
REM ============================================================
setlocal enabledelayedexpansion

echo.
echo ============================================================
echo  VORTEX BUILD v0.1.0  -  Zoe ^& Levy
echo ============================================================
echo.

REM 1. Prérequis
where cargo >nul 2>nul || (echo [X] Rust pas installe. Va sur https://rustup.rs/ && exit /b 1)
where makensis >nul 2>nul || (echo [!] NSIS pas dans PATH. Utilise portable ou installe-le.)

REM 2. Build release
echo [1/3] Compilation du moteur Rust...
cd /d "%~dp0.."
cargo build --release
if errorlevel 1 (
  echo [X] Echec de compilation
  exit /b 1
)

REM 3. Création du portable (auto-extractible 7z)
echo [2/3] Preparation du portable...
if not exist "dist" mkdir dist
copy /Y "target\release\vortex.exe" "dist\Vortex-Portable-v0.1.0.exe" >nul
echo Generation du portable auto-extractible via 7z SFX...
if exist "C:\Program Files\7-Zip\7z.exe" (
  "C:\Program Files\7-Zip\7z.exe" a -sfx7z.sfx -r "dist\Vortex-Portable.exe.tmp" "target\release\vortex.exe" "README-FIRST.txt" "LICENSE.txt"
  move /Y "dist\Vortex-Portable.exe.tmp" "dist\Vortex-Portable-v0.1.0.exe"
) else (
  echo [!] 7-Zip absent — fallback binaire seul (fonctionne quand meme depuis n'importe ou)
)

REM 4. NSIS installer
echo [3/3] Generation de l'installateur NSIS...
where makensis >nul 2>nul && (
  cd installer
  makensis vortex.nsi
  cd ..
) || echo [!] NSIS pas installe — circule dist\Vortex-Portable-v0.1.0.exe et dist\Vortex-Setup-v0.1.0.exe.skip

echo.
echo ============================================================
echo  BUILD TERMINE
echo ============================================================
echo  Sortie : dist\
echo   - Vortex-Portable-v0.1.0.exe   (utilisateur, sans install, ~2 Mo)
echo   - Vortex-Setup-v0.1.0.exe      (installateur admin, ~5 Mo)
echo ============================================================
endlocal
