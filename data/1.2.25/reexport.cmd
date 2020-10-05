@pushd "%~dp0"
@setlocal

@if not exist "%ASEPRITE%" set "ASEPRITE=%ProgramFiles(x86)%\Steam\steamapps\common\Aseprite\Aseprite.exe"
@if not exist "%ASEPRITE%" set "ASEPRITE=%ProgramFiles%\Steam\steamapps\common\Aseprite\Aseprite.exe"
@if not exist "%ASEPRITE%" echo Cannot find Aseprite.exe&& goto :err

@rmdir basic /S /Q >NUL 2>NUL
@rmdir array /S /Q >NUL 2>NUL
@rmdir hash  /S /Q >NUL 2>NUL

@mkdir basic >NUL 2>NUL
@mkdir array >NUL 2>NUL
@mkdir hash  >NUL 2>NUL

@for /f "" %%f in ('dir /b _src\*.aseprite') do @call :export %%f || goto :err

@endlocal && popd && exit /b 0
:err
@endlocal && popd && exit /b 1

:export
@call :aseprite -b "_src\%~1" --sheet "basic\%~1.png" --data "basic\%~1.json" || exit /b 1
@call :aseprite -b "_src\%~1" --sheet "array\%~1.png" --data "array\%~1.json" --format json-array --list-layers --list-tags --list-slices --all-layers || exit /b 1
@call :aseprite -b "_src\%~1" --sheet "hash\%~1.png"  --data "hash\%~1.json"  --format json-hash  --list-layers --list-tags --list-slices --all-layers || exit /b 1
@exit /b 0

:aseprite
@echo aseprite %*
@"%ASEPRITE%" %*
@exit /b %ERRORLEVEL%
