#!/usr/bin/env sh

rm -rf ./README.md
touch ./README.md

echo '# Index / TOC\n\n```' >>./README.md
tree -L 2 >>./README.md
if [[ "$(uname)" == 'Darwin' ]]; then
  # https://formulae.brew.sh/formula/coreutils
  ghead -n -2 ./README.md >./README.md.tmp
else
  head -n -2 ./README.md >./README.md.tmp
fi
mv ./README.md.tmp ./README.md
echo '```\n\n# License\n\n[WTFPL](./LICENSE.md)' >>./README.md
