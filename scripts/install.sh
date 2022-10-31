#!/usr/bin/bash

GREEN='\033[0;32m'
NC='\033[0m'
YELLOW='\033[0;33m'

printf "${NC}==>${GREEN} Installing takoyaki at $HOME.local/bin\n"

# Download the file
curl https://download.kyeboard.me/takoyaki --output ~/.local/bin/takoyaki > /dev/null 2>&1

# Add executable permissions 
chmod +x ~/.local/bin/takoyaki

printf "${NC}==>${GREEN} Initializing with a new instance...\n"

# Check if the path exists in PATH
if [[ ":$PATH:" == *":$HOME.local/bin:"* ]]; then
    # Initialize a new instance
    takoyaki init
else
    printf "\n${YELLOW}takoyaki is installed at $HOME.local/bin/ which is currently not in PATH. Fix this warning by add $HOME.local/bin to your PATH!"
fi


# takoyaki init
#
# printf "${GREEN}==>${NC} Successfully initialized a new instance!\n"
#
# printf "${YELLOW}\nIt is recommended to install GitHub plugin to show your GitHub contributions. Read more here - https://github.com/kyeboard/takoyaki/tree/main/plugins/github"
