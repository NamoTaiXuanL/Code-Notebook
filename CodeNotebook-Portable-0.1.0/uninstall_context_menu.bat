@echo off
chcp 65001 >nul 2>&1
echo ========================================
echo Code Viewer - Context Menu Uninstall
echo ========================================

echo.
echo This script will remove the Code Viewer context menu
echo.
pause

echo.
echo Removing context menu...

:: Delete context menu item
reg delete "HKCR\*\shell\OpenWithCodeNotebook" /f >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo Warning: Failed to delete context menu item, it may not exist or has been already removed
) else (
    echo Context menu item removed successfully
)

echo.
echo Context menu uninstall complete!
echo.
pause