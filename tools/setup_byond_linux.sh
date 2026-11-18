#!/bin/bash
set -euo pipefail

BYOND_MAJOR=516
BYOND_MINOR=1688

if [ -d "$HOME/BYOND/byond/bin" ] && grep -Fxq "${BYOND_MAJOR}.${BYOND_MINOR}" $HOME/BYOND/version.txt;
then
  echo "Using cached directory."
else
  echo "Setting up BYOND."
  rm -rf "$HOME/BYOND"
  mkdir -p "$HOME/BYOND"
  cd "$HOME/BYOND"
  curl -H "User-Agent: byondapi-rs CI Script" "http://www.byond.com/download/build/${BYOND_MAJOR}/${BYOND_MAJOR}.${BYOND_MINOR}_byond_linux.zip" -o byond.zip
  if [ $? -ne 0 ] || !(unzip -qt byond.zip); then
    echo "Attempting fallback mirror..."
    rm byond.zip
    curl "https://byond-builds.dm-lang.org/${BYOND_MAJOR}/${BYOND_MAJOR}.${BYOND_MINOR}_byond_linux.zip" -o byond.zip -A "byondapi-rs CI Script"
    if [ $? -ne 0 ] || !(unzip -qt byond.zip); then
      echo "Failure!"
      exit 1
    fi
  fi
  unzip byond.zip
  rm byond.zip
  cd byond
  make here
  echo "$BYOND_MAJOR.$BYOND_MINOR" > "$HOME/BYOND/version.txt"
  cd ~/
fi
