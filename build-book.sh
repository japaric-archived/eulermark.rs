#!/bin/bash

mkdir -p stage
rm -f stage/SUMMARY.md

for pid in $(ls problems); do
  readme=problems/${pid}/README.md

  if [[ ! -f ${readme} ]]; then
    continue
  fi

  cp ${readme} stage/${pid}.md

  sed -i "s:(https.*):(/../master/problems/${pid}):1" stage/${pid}.md

  echo "* [${title}](${pid}.md)" >> stage/SUMMARY.md
done

ln -sf ../src/intro.md stage/README.md

cd stage && gitbook build
