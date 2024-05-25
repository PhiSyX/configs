Invoke-Expression (&starship init powershell)

# Import the Chocolatey Profile that contains the necessary code to enable
# tab-completions to function for `choco`.
# Be aware that if you are missing these lines from your profile, tab completion
# for `choco` will not function.
# See https://ch0.co/tab-completion for details.
$ChocolateyProfile = "$env:ChocolateyInstall\helpers\chocolateyProfile.psm1"
if (Test-Path($ChocolateyProfile)) {
  Import-Module "$ChocolateyProfile"
}

Set-PSReadLineOption -PredictionSource History
Set-PSReadLineOption -PredictionViewStyle ListView
Set-PSReadLineOption -EditMode Windows

# proto
$env:PROTO_HOME = Join-Path $HOME ".proto"
$env:GOBIN = Join-Path $HOME "go" "bin"
# path
$env:PATH = @(
   ( Join-Path $env:PROTO_HOME "shims" ), 
   ( Join-Path $env:PROTO_HOME "bin" ), 
   ( Join-Path $env:GOBIN "" ),
   $env:PATH 
) -join [IO.PATH]::PathSeparator
