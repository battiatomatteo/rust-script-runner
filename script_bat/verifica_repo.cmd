@echo off
SETLOCAL ENABLEDELAYEDEXPANSION

REM Inserisci uno dei tuoi percorsi problematici qui:
set "REPO=C:\Users\baddy\Desktop\appunti\Appunti"

echo Controllo contenuto della cartella: !REPO!
echo.

if exist "!REPO!\" (
    dir /a "!REPO!"
    echo.
    if exist "!REPO!\.git\" (
        echo ✅ La cartella .git ESISTE in "!REPO!"
    ) else (
        echo ❌ La cartella .git NON è presente in "!REPO!"
    )
) else (
    echo ❌ Il percorso "!REPO!" NON esiste
)

pause
