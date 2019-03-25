Param(
    [bool]$Run = $true
)

& .\z_clean.ps1
& .\z_build.ps1 -OutputType "DLL"
& .\z_build.ps1 -OutputType "EXE"

if ($Run -eq $true) {
    & ".\linkedlist_tests.exe"
}