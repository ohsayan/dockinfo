# Installation script in $HOME/bin for debian systems ONLY
FILE=./dockinfo
if [ "$(grep -Ei 'debian|buntu|mint' /etc/*release)" ]; then
    if test -f "$FILE"; then
        mkdir -p $HOME/bin
        cp ./dockinfo $HOME/bin
        if [[ ":$PATH:" == *":$HOME/bin:"* ]]; then
            echo 'Done!'
            exit 0
        else
            echo 'export PATH="$HOME/bin/:$PATH"' >> $HOME/.bashrc
            echo 'Done!'
            exit 0
        fi
    else
        echo "This directory does not contain the dockinfo binary!"
        exit 1
    fi
else
    echo "This is not a Debian distribution"
    exit 1
fi
