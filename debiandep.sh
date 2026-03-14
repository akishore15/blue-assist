sudo apt update
sudo apt upgrade
sudo apt install software-properties-common -y
sudo add-apt-repository ppa:deadsnakes/ppa
sudo apt update
sudo apt install python3.13
curl -sS https://bootstrap.pypa.io/get_pip.py | python3.13
pip install pvporcupine faster-whisper sounddevice numpy pyttsx3 pyautogui perplexity-api requests
curl -sSfL https://ollama.com/download.sh | sh
ollama pull llama3
