; Vortex Installer — NSIS 3.x
; Génère : Vortex-Setup-v0.1.0.exe   (installateur admin)
; Génère : Vortex-Portable-v0.1.0.exe (utilisateur, sans install)
;
; Usage : makensis vortex.nsi

!include "MUI2.nsh"
!include "x64.nsh"

Name "Vortex — Symbiotic PC Optimizer"
OutFile "..\dist\Vortex-Setup-v0.1.0.exe"
InstallDir "$PROGRAMFILES64\Vortex"
InstallDirRegKey HKLM "Software\Vortex" "InstallDir"
RequestExecutionLevel admin
ShowInstDetails show
ShowUninstDetails show

BrandingText "Vortex — Zoe & Lévy  ·  ère simbiotique"

!define VORTEX_VERSION "0.1.0"
!define VORTEX_PUBLISHER "Vortex Project"

VIProductVersion "${VORTEX_VERSION}.0"
VIAddVersionKey  "ProductName"   "Vortex"
VIAddVersionKey  "ProductVersion" "${VORTEX_VERSION}"
VIAddVersionKey  "Company"       "${VORTEX_PUBLISHER}"
VIAddVersionKey  "FileDescription" "Vortex PC Optimizer Installer"
VIAddVersionKey  "LegalCopyright" "© 2026 Zoe & Lévy"
VIAddVersionKey  "Comments"      "Optimise Windows 11 selon ton matériel"

!macro VORTEX_GUI_PAGES
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE.txt"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_TITLE "Vortex est installé"
!define MUI_FINISHPAGE_TEXT "Vortex est prêt.$\r$\n$\r$\nCoche 'Lancer Vortex' puis profite."
!define MUI_FINISHPAGE_RUN "$INSTDIR\vortex.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Lancer Vortex maintenant"
!define MUI_FINISHPAGE_SHOWREADME "$INSTDIR\README-FIRST.txt"
!define MUI_FINISHPAGE_SHOWREADME_TEXT "Afficher le README"
!define MUI_FINISHPAGE_SHOWREADME_NOTCHECKED
!insertmacro MUI_PAGE_FINISH
!macroend

!insertmacro VORTEX_GUI_PAGES

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "French"

Section "Vortex (requis)" SecVortex
  SectionIn RO
  SetOutPath "$INSTDIR"
  File "..\target\release\vortex.exe"
  File "LICENSE.txt"
  File "README-FIRST.txt"

  ; Créer lien menu démarrer
  CreateDirectory "$SMPROGRAMS\Vortex"
  CreateShortcut "$SMPROGRAMS\Vortex\Vortex.lnk"          "$INSTDIR\vortex.exe"
  CreateShortcut "$SMPROGRAMS\Vortex\Vortex Dash.lnk"     "$INSTDIR\vortex.exe" "dash"
  CreateShortcut "$SMPROGRAMS\Vortex\Vortex Restore.lnk"  "$INSTDIR\vortex.exe" "restore"
  CreateShortcut "$SMPROGRAMS\Vortex\Dashboard.lnk"       "$INSTDIR\vortex.exe" "info"

  ; Bureau
  CreateShortcut "$DESKTOP\Vortex.lnk" "$INSTDIR\vortex.exe"

  ; Uninstall
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vortex" "DisplayName" "Vortex"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vortex" "DisplayVersion" "${VORTEX_VERSION}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vortex" "Publisher" "${VORTEX_PUBLISHER}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vortex" "UninstallString" "$INSTDIR\Uninstall.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vortex" "InstallLocation" "$INSTDIR"
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vortex" "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vortex" "NoRepair" 1
SectionEnd

Section "Au démarrage de Windows (mode Protect)" SecProtect
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "VortexProtect" '"$INSTDIR\vortex.exe" protect'
SectionEnd

Section "Liens web" SecWeb
  CreateShortcut "$SMPROGRAMS\Vortex\Site.lnk" "https://github.com/lvs0/vortex"
SectionEnd

Section "Uninstall"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Vortex"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "VortexProtect"
  RMDir /r "$SMPROGRAMS\Vortex"
  Delete "$DESKTOP\Vortex.lnk"
  RMDir /r "$INSTDIR"
SectionEnd
