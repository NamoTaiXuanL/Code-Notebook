# Code Viewer Windows 11 Context Menu Setup
# 此脚本专门用于在Windows 11一级右键菜单中添加代码查看器

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Code Viewer - Windows 11 Menu Setup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 检查Windows版本
$OSVersion = [System.Environment]::OSVersion.Version
if ($OSVersion.Major -lt 10 -or ($OSVersion.Major -eq 10 -and $OSVersion.Build -lt 22000)) {
    Write-Host "警告：检测到您的Windows版本可能不是Windows 11" -ForegroundColor Yellow
    Write-Host "此脚本专为Windows 11设计，在其他版本上可能无法正常工作" -ForegroundColor Yellow
    Write-Host ""
    $continue = Read-Host "是否继续？(Y/N)"
    if ($continue -ne "Y" -and $continue -ne "y") {
        exit
    }
}

# 检查是否以管理员身份运行
if (-NOT ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "错误：此脚本需要管理员权限运行！" -ForegroundColor Red
    Write-Host "请右键点击此脚本，选择'以管理员身份运行'" -ForegroundColor Yellow
    Read-Host "按任意键退出"
    exit 1
}

# 获取当前目录
$InstallDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ExePath = Join-Path $InstallDir "code_notebook.exe"

Write-Host "检查程序文件..." -ForegroundColor Yellow

# 检查exe文件是否存在
if (-not (Test-Path $ExePath)) {
    Write-Host "错误：未找到 $ExePath" -ForegroundColor Red
    Write-Host "请确保在正确的目录中运行此脚本" -ForegroundColor Yellow
    Read-Host "按任意键退出"
    exit 1
}

Write-Host "程序路径：$ExePath" -ForegroundColor Green
Write-Host ""
Write-Host "正在为Windows 11一级菜单注册右键菜单..." -ForegroundColor Yellow

try {
    # Windows 11 一级菜单注册表路径
    $ExtendedKeyPath = "HKLM:\SOFTWARE\Classes\*\shell\OpenWithCodeNotebook"
    $LegacyKeyPath = "HKLM:\SOFTWARE\Classes\*\shell\OpenWithCodeNotebook_Legacy"

    # 创建一级菜单项（ExtendedSupported = 0 表示显示在一级菜单）
    New-Item -Path $ExtendedKeyPath -Force | Out-Null
    Set-ItemProperty -Path $ExtendedKeyPath -Name "(Default)" -Value "Open with Code Viewer" -Force
    Set-ItemProperty -Path $ExtendedKeyPath -Name "Icon" -Value $ExePath -Force

    # 关键：设置 ExtendedSupported = 0，这样菜单项会显示在Windows 11的一级菜单中
    Set-ItemProperty -Path $ExtendedKeyPath -Name "ExtendedSupported" -Value 0 -Type DWORD -Force

    # 创建命令
    New-Item -Path "$ExtendedKeyPath\command" -Force | Out-Null
    Set-ItemProperty -Path "$ExtendedKeyPath\command" -Name "(Default)" -Value "`"$ExePath`" `"%1`"" -Force

    # 同时创建旧版菜单项作为备用（兼容性）
    New-Item -Path $LegacyKeyPath -Force | Out-Null
    Set-ItemProperty -Path $LegacyKeyPath -Name "(Default)" -Value "Open with Code Viewer (Legacy)" -Force
    Set-ItemProperty -Path $LegacyKeyPath -Name "Icon" -Value $ExePath -Force

    New-Item -Path "$LegacyKeyPath\command" -Force | Out-Null
    Set-ItemProperty -Path "$LegacyKeyPath\command" -Name "(Default)" -Value "`"$ExePath`" `"%1`"" -Force

    # 为目录也添加右键菜单支持
    $FolderKeyPath = "HKLM:\SOFTWARE\Classes\Directory\shell\OpenWithCodeNotebook"
    New-Item -Path $FolderKeyPath -Force | Out-Null
    Set-ItemProperty -Path $FolderKeyPath -Name "(Default)" -Value "Open with Code Viewer" -Force
    Set-ItemProperty -Path $FolderKeyPath -Name "Icon" -Value $ExePath -Force
    Set-ItemProperty -Path $FolderKeyPath -Name "ExtendedSupported" -Value 0 -Type DWORD -Force

    New-Item -Path "$FolderKeyPath\command" -Force | Out-Null
    Set-ItemProperty -Path "$FolderKeyPath\command" -Name "(Default)" -Value "`"$ExePath`" `"%1`"" -Force

    # 为桌面背景也添加右键菜单支持
    $DesktopKeyPath = "HKLM:\SOFTWARE\Classes\Directory\Background\shell\OpenWithCodeNotebook"
    New-Item -Path $DesktopKeyPath -Force | Out-Null
    Set-ItemProperty -Path $DesktopKeyPath -Name "(Default)" -Value "Open Code Viewer" -Force
    Set-ItemProperty -Path $DesktopKeyPath -Name "Icon" -Value $ExePath -Force
    Set-ItemProperty -Path $DesktopKeyPath -Name "ExtendedSupported" -Value 0 -Type DWORD -Force

    New-Item -Path "$DesktopKeyPath\command" -Force | Out-Null
    Set-ItemProperty -Path "$DesktopKeyPath\command" -Name "(Default)" -Value "`"$ExePath`"" -Force

    Write-Host ""
    Write-Host "✅ Windows 11一级右键菜单注册成功！" -ForegroundColor Green
    Write-Host ""
    Write-Host "现在您可以：" -ForegroundColor Cyan
    Write-Host "1. 右键点击任意文件，直接看到 'Open with Code Viewer'" -ForegroundColor White
    Write-Host "2. 右键点击文件夹，也能看到菜单项" -ForegroundColor White
    Write-Host "3. 在桌面空白处右键，可以启动代码查看器" -ForegroundColor White
    Write-Host ""
    Write-Host "注意：如果修改后没有立即生效，请尝试：" -ForegroundColor Yellow
    Write-Host "- 重启Windows资源管理器" -ForegroundColor White
    Write-Host "- 或者重启电脑" -ForegroundColor White
    Write-Host ""
    Write-Host "如需卸载，请运行 uninstall_win11_menu.ps1" -ForegroundColor Yellow

} catch {
    Write-Host "错误：注册右键菜单失败！" -ForegroundColor Red
    Write-Host "错误信息：$($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""
Read-Host "按任意键退出"