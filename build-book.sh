#!/bin/bash

repo=https://github.com/japaric/eulermark.rs/tree/master

mkdir -p stage
rm -f stage/SUMMARY.md

for pid in $(ls problems); do
  readme=problems/${pid}/README.md

  if [[ ! -f ${readme} ]]; then
    continue
  fi

  title=$(head -1 problems/${pid}/README.md | cut -d [ -f2 | cut -d ] -f1)

  cp ${readme} stage/${pid}.md

  sed -i "s,(.*project.*),(${repo}/problems/${pid})," stage/${pid}.md

  echo "* [${title}](${pid}.md)" >> stage/SUMMARY.md
done

ln -sf ../src/intro.md stage/README.md

cd stage && gitbook build
