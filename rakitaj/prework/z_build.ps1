$env:INCLUDE = "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\ATLMFC\include;C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\include;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.6.1\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\ucrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\winrt;C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\cppwinrt"
$env:LIB = "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\ATLMFC\lib\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\lib\x86;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.6.1\lib\um\x86;C:\Program Files (x86)\Windows Kits\10\lib\10.0.17763.0\ucrt\x86;C:\Program Files (x86)\Windows Kits\10\lib\10.0.17763.0\um\x86;"
$env:LIBPATH = "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\ATLMFC\lib\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\lib\x86;C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\lib\x86\store\references;C:\Program Files (x86)\Windows Kits\10\UnionMetadata\10.0.17763.0;C:\Program Files (x86)\Windows Kits\10\References\10.0.17763.0;C:\Windows\Microsoft.NET\Framework\v4.0.30319;"


Set-Location "~\projects\crafting\prework"

$cl = "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Tools\MSVC\14.16.27023\bin\Hostx86\x86\cl.exe"

# Alternate syntax for creating DLL.
# & $cl "/LD" "linkedlist.c" "/link" "/out:LinkedList.dll"
& $cl "/LD" "linkedlist.c" "/FeLinkedList.dll"
& $cl "linkedlist_tests.c"
