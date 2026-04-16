#!/bin/bash
clear
echo -e "\e[32mAvvio AI Lab - Miniconda\e[0m"

# Funzione di caricamento con barra fluida
loading() {
    local msg="$1"
    clear
    echo "$msg..."
    for i in {1..100}; do
        clear
        echo "$msg..."
        echo -ne "Caricamento: $i%\r"
        sleep 0.02
    done
    echo -e "\nCompletato 100%!"
    echo
}

# Attivazione ambiente Miniconda
loading "Attivazione ambiente Miniconda"
source ~/miniconda3/etc/profile.d/conda.sh
conda activate ai-lab

# Accesso alla cartella AI_Lab
loading "Accesso alla cartella AI_Lab"
cd ~/AI_Lab || exit

# Avvio di Jupyter Notebook
loading "Avvio di Jupyter Notebook"
jupyter notebook
