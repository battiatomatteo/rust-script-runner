@echo off
setlocal enabledelayedexpansion

:: Chiedi nome cartella
set /p foldername=Nome (anche parziale) della cartella da cercare: 

:: Percorsi di ricerca
set "search1=%USERPROFILE%\Desktop"
set "search2=%USERPROFILE%\Documents"
set "search3=C:\"

:: File temporaneo
set "alltemp=%TEMP%\found_paths.txt"
if exist "%alltemp%" del "%alltemp%"

echo.
echo 🔍 Cercando cartelle che contengono "%foldername%"...

:: Ricerca in tutti i percorsi
for %%d in ("%search1%" "%search2%" "%search3%") do (
    echo Cerca in %%~d ...
    dir /b /s /ad "%%~d\*%foldername%*" 2>nul >> "%alltemp%"
)

:: Conta risultati
set count=0
for /f "delims=" %%f in (%alltemp%) do (
    set /a count+=1
    set "result!count!=%%f"
)

if %count%==0 (
    echo ❌ Nessuna cartella trovata con "%foldername%".
    goto end
)

echo.
echo ✅ Trovate %count% cartelle:
for /L %%i in (1,1,%count%) do (
    echo %%i. !result%%i!
)

echo.
set /p choice=Numero della cartella da aprire in VSCode: 
set "selected=!result%choice%!"

if defined selected (
    echo Aprendo "!selected!" in Visual Studio Code...
    code "!selected!"
) else (
    echo ❌ Scelta non valida.
)

:end
:: Pulisci file temporaneo
if exist "%alltemp%" del "%alltemp%"
pause
