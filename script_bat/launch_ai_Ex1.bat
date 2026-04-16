@echo off
title Avvio AI Project - Miniconda
color 0A

REM Attivazione ambiente Miniconda
CALL "%USERPROFILE%\miniconda3\Scripts\activate.bat"
CALL conda activate ai-lab

REM Accesso alla cartella Progetto AI
cd /d "%USERPROFILE%\Ex-1-Progetto"

REM Avvio di Jupyter Notebook
CALL jupyter notebook

exit /b
