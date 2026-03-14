sudo dnf upgrade -y

sudo dnf install python3 python3-pip python3-devel portaudio-devel \
gcc gcc-c++ make curl git -y

python3 -m pip install --upgrade pip

pip install pvporcupine faster-whisper sounddevice numpy pyttsx3 pyautogui perplexity-api requests

curl -sSfL https://ollama.com/download.sh | sh
ollama pull llama3
