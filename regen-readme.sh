#!/usr/bin/env sh

rm -rf ./README.md
touch ./README.md

echo '# Index / TOC\n\n```' >>./README.md
tree -L 2 >>./README.md
echo '```\n\n# License\n\n[WTFPL](./LICENSE.md)' >>./README.md
