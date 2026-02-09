$urls = @(
    "https://github.com/Raihanat-debug/cv-tailor/releases/download/v1.0.6/cv-tailor-x86_64-unknown-linux-musl.tar.gz",
    "https://github.com/Raihanat-debug/cv-tailor/releases/download/v1.0.6/cv-tailor-aarch64-unknown-linux-musl.tar.gz"
)

$destinations = @(
    "target/cv-tailor-x86_64-unknown-linux-musl.tar.gz",
    "target/cv-tailor-aarch64-unknown-linux-musl.tar.gz"
)

Write-Host "Waiting for GitHub Release v1.0.6 artifacts to be ready... (This may take a few minutes)"

for ($i = 0; $i -lt $urls.Count; $i++) {
    $url = $urls[$i]
    $dest = $destinations[$i]
    
    $retries = 0
    $max_retries = 20 # Wait up to 10 minutes (20 * 30s)
    $downloaded = $false

    while (-not $downloaded -and $retries -lt $max_retries) {
        try {
            Write-Host "Checking $url... (Attempt $($retries + 1)/$max_retries)"
            # Check if file exists (HEAD request)
            $request = [System.Net.WebRequest]::Create($url)
            $request.Method = "HEAD"
            $response = $request.GetResponse()
            $response.Close()

            # If we get here, the file exists. Download it.
            Write-Host "File found! Downloading to $dest..."
            Invoke-WebRequest -Uri $url -OutFile $dest -ErrorAction Stop
            Write-Host "Successfully downloaded: $dest"
            $downloaded = $true
        }
        catch {
            # 404 or other error
            Write-Host "File not ready yet. Waiting 30 seconds..."
            Start-Sleep -Seconds 30
            $retries++
        }
    }

    if (-not $downloaded) {
        Write-Error "Timed out waiting for $url. Please check GitHub Actions status manually."
    }
}
