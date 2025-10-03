Remove-Item ./export -Force -Recurse
New-Item -ItemType Directory -Path ./export

./emm.exe run `
    --export `
    --game-directory . `
    --export-directory ./export `
    --force
