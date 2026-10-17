; Whisper Pill – NSIS Windows Installations-Assistent (MUI2)
; Kompiliert unter Linux oder Windows via `makensis packaging/windows/installer.nsi`

!include "MUI2.nsh"

!define APP_NAME "Whisper Pill"
!define APP_VERSION "0.1.0"
!define APP_PUBLISHER "whisper-pill contributors"
!define APP_EXE "whisper_pill.exe"

Name "${APP_NAME} v${APP_VERSION}"
OutFile "../../dist/Whisper_Pill_Setup_x86_64.exe"
InstallDir "$LOCALAPPDATA\Programs\WhisperPill"
InstallDirRegKey HKCU "Software\WhisperPill" "Install_Dir"
RequestExecutionLevel user

; Moderne Oberflaechen-Einstellungen
!define MUI_ABORTWARNING
!define MUI_FINISHPAGE_RUN "$INSTDIR\${APP_EXE}"
!define MUI_FINISHPAGE_RUN_TEXT "Whisper Pill jetzt im System-Tray starten"

; Wizard-Seiten
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "../../LICENSE"
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

; Deinstallations-Seiten
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "German"

Section "Whisper Pill Hauptprogramm (Erforderlich)" SecCore
  SectionIn RO
  SetOutPath "$INSTDIR"

  ; Hauptprogramm und Lizenz kopieren
  File "../../app/target/x86_64-pc-windows-gnu/release/${APP_EXE}"
  File "../../LICENSE"

  ; Installationspfad und Uninstaller in der Benutzer-Registry hinterlegen
  WriteRegStr HKCU "Software\WhisperPill" "Install_Dir" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\WhisperPill" "DisplayName" "${APP_NAME}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\WhisperPill" "DisplayVersion" "${APP_VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\WhisperPill" "Publisher" "${APP_PUBLISHER}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\WhisperPill" "UninstallString" '"$INSTDIR\Uninstall.exe"'
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\WhisperPill" "NoModify" 1
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\WhisperPill" "NoRepair" 1

  WriteUninstaller "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Startmenü- & Desktop-Verknüpfung" SecShortcuts
  CreateDirectory "$SMPROGRAMS\${APP_NAME}"
  CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}"
  CreateShortcut "$SMPROGRAMS\${APP_NAME}\Whisper Pill deinstallieren.lnk" "$INSTDIR\Uninstall.exe"
  CreateShortcut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}"
SectionEnd

Section "Mit Windows starten (Autostart)" SecAutostart
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "WhisperPill" '"$INSTDIR\${APP_EXE}"'
SectionEnd

LangString DESC_SecCore ${LANG_GERMAN} "Installiert die native Rust-Anwendung Whisper Pill (100 % lokale KI-Spracherkennung)."
LangString DESC_SecShortcuts ${LANG_GERMAN} "Erstellt Verknüpfungen im Windows-Startmenü und auf dem Desktop."
LangString DESC_SecAutostart ${LANG_GERMAN} "Startet Whisper Pill automatisch bei der Windows-Anmeldung im Hintergrund."

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecCore} $(DESC_SecCore)
  !insertmacro MUI_DESCRIPTION_TEXT ${SecShortcuts} $(DESC_SecShortcuts)
  !insertmacro MUI_DESCRIPTION_TEXT ${SecAutostart} $(DESC_SecAutostart)
!insertmacro MUI_FUNCTION_DESCRIPTION_END

Section "Uninstall"
  ; Laufende Instanz sauber beenden
  nsExec::ExecToLog 'taskkill /F /IM ${APP_EXE}'

  ; Autostart- und Deinstallations-Registry-Schluessel entfernen
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "WhisperPill"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\WhisperPill"
  DeleteRegKey HKCU "Software\WhisperPill"

  ; Verknuepfungen und Programmdateien entfernen
  Delete "$DESKTOP\${APP_NAME}.lnk"
  Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
  Delete "$SMPROGRAMS\${APP_NAME}\Whisper Pill deinstallieren.lnk"
  RMDir "$SMPROGRAMS\${APP_NAME}"

  Delete "$INSTDIR\${APP_EXE}"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"
SectionEnd
