; Nexora installer (NSIS 3). Per-user install, no admin rights needed.
;
; The name shown to people is "Nexora" (the 0.1.x versions from 0.1.2 on said "Nexora"); the install folder,
; the uninstall key, the Run value and nexora.exe keep the name "Nexora", so older installs upgrade in place.
;
; Build (Linux/macOS/Windows with makensis):
;   makensis -DEXE=/abs/path/nexora.exe -DOUTDIR=/abs/path/dist installer/nexora.nsi
; tools/package.sh does this for you.

; Native 64-bit installer (Nexora itself is x64-only).
Target amd64-unicode
SetCompressor /SOLID lzma

!define APPNAME "Nexora"
!define VERSION "0.1.0"
!define PUBLISHER "Jatevo Labs"
!define UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\Nexora"
!define RUN_KEY "Software\Microsoft\Windows\CurrentVersion\Run"

!ifndef EXE
  !error "Pass -DEXE=<path to nexora.exe>"
!endif
!ifndef OUTDIR
  !define OUTDIR "."
!endif

Name "${APPNAME}"
OutFile "${OUTDIR}/Nexora-Setup-${VERSION}.exe"
RequestExecutionLevel user
InstallDir "$LOCALAPPDATA\Programs\Nexora"
InstallDirRegKey HKCU "Software\Nexora" "InstallDir"
BrandingText "${APPNAME} ${VERSION}"

VIProductVersion "0.1.0.0"
VIAddVersionKey "ProductName" "${APPNAME}"
VIAddVersionKey "CompanyName" "${PUBLISHER}"
VIAddVersionKey "FileDescription" "Nexora installer"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "ProductVersion" "${VERSION}"
VIAddVersionKey "LegalCopyright" "Copyright (C) 2026 ${PUBLISHER}"

!include "MUI2.nsh"
!include "LogicLib.nsh"
!include "WinMessages.nsh"
!include "FileFunc.nsh"
!include "Sections.nsh"

; Ask a running Nexora to quit (it saves its data on WM_CLOSE), wait up to 3 s,
; then force-close with the fully qualified system taskkill.
!macro StopNexora
  FindWindow $0 "Nexora.Island"
  ${If} $0 <> 0
    SendMessage $0 ${WM_CLOSE} 0 0 /TIMEOUT=2000
    StrCpy $1 0
    ${DoWhile} $0 <> 0
      ${If} $1 >= 30
        ${Break}
      ${EndIf}
      Sleep 100
      FindWindow $0 "Nexora.Island"
      IntOp $1 $1 + 1
    ${Loop}
  ${EndIf}
  nsExec::Exec '"$SYSDIR\taskkill.exe" /IM nexora.exe /F'
  Sleep 200
!macroend
!define MUI_ICON "../crates/nexora/res/nexora.ico"
!define MUI_UNICON "../crates/nexora/res/nexora.ico"
; Nova hanging from the island on the welcome and finish pages, Nexora's logo in the header (tools/make_brand.py).
!define MUI_WELCOMEFINISHPAGE_BITMAP "art/nsis-welcome.bmp"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_RIGHT
!define MUI_HEADERIMAGE_BITMAP "art/nsis-header.bmp"
!define MUI_ABORTWARNING
!define MUI_COMPONENTSPAGE_SMALLDESC
!define MUI_FINISHPAGE_RUN "$INSTDIR\nexora.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Launch Nexora now"
!define MUI_WELCOMEPAGE_TITLE "Welcome to Nexora"
!define MUI_WELCOMEPAGE_TEXT "Nexora puts a living notch at the top of your screen: music with synced lyrics, your day, a focus timer, your steps, a file shelf, tasks and more.$\r$\n$\r$\nIt installs just for you and doesn't need administrator rights."

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

Section "Nexora" SecMain
  SectionIn RO
  ; Close a running copy so the exe can be replaced.
  !insertmacro StopNexora
  SetOutPath "$INSTDIR"
  File "/oname=nexora.exe" "${EXE}"
  ; The licenses of what nexora.exe is built from (mecab-ipadic asks for its notice in every copy).
  File "../THIRD-PARTY-NOTICES.txt"
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  ; Older shortcuts: "Nexora" (0.1.0, 0.1.1) and "Nexora" (0.1.2 to the last 0.1.x).
  Delete "$SMPROGRAMS\Nexora.lnk"
  Delete "$SMPROGRAMS\Nexora.lnk"
  CreateShortcut "$SMPROGRAMS\${APPNAME}.lnk" "$INSTDIR\nexora.exe" "" "$INSTDIR\nexora.exe" 0
  WriteRegStr HKCU "Software\Nexora" "InstallDir" "$INSTDIR"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayName" "${APPNAME}"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "${UNINST_KEY}" "Publisher" "${PUBLISHER}"
  WriteRegStr HKCU "${UNINST_KEY}" "DisplayIcon" "$INSTDIR\nexora.exe"
  WriteRegStr HKCU "${UNINST_KEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "${UNINST_KEY}" "UninstallString" '"$INSTDIR\Uninstall.exe"'
  WriteRegStr HKCU "${UNINST_KEY}" "QuietUninstallString" '"$INSTDIR\Uninstall.exe" /S'
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoModify" 1
  WriteRegDWORD HKCU "${UNINST_KEY}" "NoRepair" 1
  WriteRegDWORD HKCU "${UNINST_KEY}" "EstimatedSize" 7680
SectionEnd

Section "Start Nexora with Windows" SecAutostart
  ; Same value the app's own "Start with Windows" toggle writes.
  WriteRegStr HKCU "${RUN_KEY}" "Nexora" '"$INSTDIR\nexora.exe"'
SectionEnd

; The app's own update runs this Setup silently: `/S /reopen` (crates/nexora/src/services/update.rs).
Function .onInit
  ; Silent: Start with Windows stays as the person set it (the Run value already points here).
  ${If} ${Silent}
    !insertmacro UnselectSection ${SecAutostart}
  ${EndIf}
FunctionEnd

Function .onInstSuccess
  ; The finish page's "Launch Nexora now" never shows in silent mode: /reopen starts it.
  ${If} ${Silent}
    ${GetParameters} $R0
    ClearErrors
    ${GetOptions} $R0 "/reopen" $R1
    ${IfNot} ${Errors}
      Exec '"$INSTDIR\nexora.exe"'
    ${EndIf}
  ${EndIf}
FunctionEnd

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecMain} "The Nexora app and a Start Menu shortcut."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecAutostart} "Open Nexora automatically when you sign in. You can change this later from its menu."
!insertmacro MUI_FUNCTION_DESCRIPTION_END

Section "Uninstall"
  !insertmacro StopNexora
  Delete "$INSTDIR\nexora.exe"
  Delete "$INSTDIR\THIRD-PARTY-NOTICES.txt"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"
  Delete "$SMPROGRAMS\${APPNAME}.lnk"
  Delete "$SMPROGRAMS\Nexora.lnk"
  Delete "$SMPROGRAMS\Nexora.lnk"
  DeleteRegValue HKCU "${RUN_KEY}" "Nexora"
  DeleteRegKey HKCU "${UNINST_KEY}"
  DeleteRegKey HKCU "Software\Nexora"
  ; Cache and log (lyrics lookups name the songs you played) go too.
  RMDir /r "$LOCALAPPDATA\Nexora"
  ; Your settings and data (tasks, shelf, focus history) in %APPDATA%\Nexora are kept.
SectionEnd
