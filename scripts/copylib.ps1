# $CURR_VERSION = "smtc_windows-v$(Select-String -Path packages/smtc_windows/pubspec.yaml -Pattern '^version: ' | Select-Object -ExpandProperty LineNumber | ForEach-Object { (Get-Content packages/smtc_windows/pubspec.yaml)[$_ - 1] -replace 'version: ', '' })"

# Copy-Item platform-build/other.tar.gz packages/smtc_windows/windows/$CURR_VERSION.tar.gz

Copy-Item platform-build/smtc_windows.dll packages/smtc_windows/windows/smtc_windows.dll