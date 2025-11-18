# Build both English and Chinese documentation

Write-Host "Building English documentation..." -ForegroundColor Green
mdbook build

Write-Host "Building Chinese documentation..." -ForegroundColor Green
mdbook build --config book.zh.toml

Write-Host "Documentation built successfully!" -ForegroundColor Green
Write-Host "English: book/build/index.html"
Write-Host "Chinese: book/build/zh/index.html"
