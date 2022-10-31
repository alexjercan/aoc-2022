#!/bin/zsh

if ! command -v fswatch &> /dev/null; then
  echo "$0": Please install fswatch. >&2
  exit 1
fi

t="$(make 2>&1)"
echo "[2J[HAOC Watcher\n$t"

fswatch -m poll_monitor -0 -r **/*.rs \
    | xargs -0 -I {} python3 -c "import pathlib;print(pathlib.Path('{}').stem)" \
    | xargs -I {} sh -c 't="$(make 2>&1 {})"; echo "[2J[HAOC Watcher\n$t"'

