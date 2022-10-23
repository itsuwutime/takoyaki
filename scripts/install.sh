GREEN='\033[0;32m'
NC='\033[0m'
YELLOW='\033[1;33m'

printf "${GREEN}==>${NC} Installing takoyaki at ~/.local/bin\n"

curl https://raw.githubusercontent.com/kyeboard/takoyaki/main/bin/takoyaki --output ~/.local/bin/takoyaki

chmod +x ~/.local/bin/takoyaki

printf "${GREEN}==>${NC} Initializing with a new instance...\n"

takoyaki init

printf "${GREEN}==>${NC} Successfully initialized a new instance!\n"

printf "${YELLOW}\nIt is recommended to install GitHub plugin to show your GitHub contributions. Read more here - https://github.com/kyeboard/takoyaki/tree/main/plugins/github"
