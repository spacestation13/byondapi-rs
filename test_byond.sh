#!/bin/bash
set -euo pipefail

export BYOND_MAJOR=515
export BYOND_MINOR=1620

if [ -d "$HOME/BYOND/byond/bin" ] && grep -Fxq "${BYOND_MAJOR}.${BYOND_MINOR}" $HOME/BYOND/version.txt;
then
  echo "Using cached directory."
else
  echo "Setting up BYOND."
  rm -rf "$HOME/BYOND"
  mkdir -p "$HOME/BYOND"
  cd "$HOME/BYOND"
  curl "http://www.byond.com/download/build/${BYOND_MAJOR}/${BYOND_MAJOR}.${BYOND_MINOR}_byond.zip" -o byond.zip
  unzip byond.zip
  rm byond.zip
  cd byond
  echo "$BYOND_MAJOR.$BYOND_MINOR" > "$HOME/BYOND/version.txt"
  cd ~/
fi
mkdir -p "$GITHUB_WORKSPACE/crates/byondapi-rs-test/dm_project/byond"
cp -r "$HOME/BYOND/byond/bin" "$GITHUB_WORKSPACE/crates/byondapi-rs-test/dm_project/byond"
