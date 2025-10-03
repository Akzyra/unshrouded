Remove-Item ./unpacked -Force -Recurse
New-Item -ItemType Directory -Path ./unpacked

./kfc-parser.exe unpack `
    --game-directory . `
    --output ./unpacked
