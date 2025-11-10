@echo off
chcp 65001 >nul 2>&1
echo ========================================
echo Code Viewer - Context Menu Setup
echo ========================================

echo.
echo This script will register the Code Viewer context menu
echo.
pause

echo.
echo Registering context menu...

:: Get current directory
set "INSTALL_DIR=%~dp0"
set "INSTALL_DIR=%INSTALL_DIR:~0,-1%"
set "EXE_PATH=%INSTALL_DIR%\code_notebook.exe"

:: Check if exe file exists
if not exist "%EXE_PATH%" (
    echo Error: %EXE_PATH% not found
    echo Please make sure you are running this script in the correct directory
    pause
    exit /b 1
)

echo Program path: %EXE_PATH%

:: Add context menu item
reg add "HKCR\*\shell\OpenWithCodeNotebook" /v "" /t REG_SZ /d "Open with Code Viewer" /f >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo Error: Failed to add context menu item! Please run as administrator.
    pause
    exit /b 1
)

reg add "HKCR\*\shell\OpenWithCodeNotebook\command" /v "" /t REG_SZ /d "\"%EXE_PATH%\" \"%%1\"" /f >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo Error: Failed to add command! Please run as administrator.
    pause
    exit /b 1
)

:: Add icon
reg add "HKCR\*\shell\OpenWithCodeNotebook" /v "Icon" /t REG_SZ /d "%EXE_PATH%" /f >nul 2>&1

echo.
echo Context menu registered successfully!
echo.
echo Now you can:
echo 1. Right-click on any file
echo 2. Select "Open with Code Viewer"
echo 3. View the file content in Code Viewer
echo.
echo To uninstall context menu, run uninstall_context_menu.bat
echo.
pause