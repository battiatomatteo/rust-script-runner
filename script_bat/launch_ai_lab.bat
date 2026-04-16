@echo off
title Avvio AI Lab - Miniconda
color 0A

REM Funzione di caricamento con barra fluida
call :loading "Attivazione ambiente Miniconda"
CALL "%USERPROFILE%\miniconda3\Scripts\activate.bat"
CALL conda activate ai-lab

call :loading "Accesso alla cartella AI_Lab"
cd /d "%USERPROFILE%\AI_Lab"

call :loading "Avvio di Jupyter Notebook"
CALL jupyter notebook
goto :eof

:loading
setlocal enabledelayedexpansion
set "msg=%~1"

cls
echo %msg%...

for /L %%i in (1,1,100) do (
    cls
    echo %msg%...
    echo Caricamento: %%i%% 
    timeout /nobreak /t 0 >nul
)

echo Completato 100%%!
echo.
endlocal
goto :eof
