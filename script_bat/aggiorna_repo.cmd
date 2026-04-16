@echo off
echo ============================
echo Aggiornamento repository Git
echo ============================

REM Lista dei percorsi (ognuno racchiuso tra virgolette)
set REPOS="C:\Users\baddy\Desktop\appunti\Appunti"
"C:\Users\baddy\Desktop\RepoProgJavaFX\Progetto-Java-Fx"
"C:\Users\baddy\Desktop\app\App_Note"
"C:\Users\baddy\Desktop\Calendar\Calendar-dist"
"C:\Users\baddy\Desktop\linguaggi_programmazione\c\repositoryC\Lavori_C"
"C:\Users\baddy\Desktop\linguaggi_programmazione\Dart\RepositoryD\Lavori_Dart"
"C:\Users\baddy\Desktop\linguaggi_programmazione\Python\repository_bot\Bot_Presentazione_Telegram"
"C:\Users\baddy\Desktop\linguaggi_programmazione\Python\Bot_Clan"

REM Ciclo su ogni percorso
for %%R in (%REPOS%) do (
    echo.
    echo 🔍 Verifico la cartella: %%~R
    if exist "%%~R\.git\" (
        echo ✅ Repository trovata! Entro nella cartella...
        pushd "%%~R"
        echo ▶️  Eseguo: git pull
        git pull
        popd
    ) else (
        echo ⚠️  ATTENZIONE: "%%~R" NON è una repository Git
    )
)

echo.
echo ✅ Controllo completato.
echo Premi un tasto per chiudere...
pause >nul
