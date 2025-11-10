; 编码：UTF-8
; NSIS安装脚本：代码查看器

!include "MUI2.nsh"
!include "WinVer.nsh"

; 定义基本信息
!define APPNAME "代码查看器"
!define APPVERSION "0.1.0"
!define APPPUBLISHER "code_notebook项目组"
!define APPEXE "code_notebook.exe"
!define APPID "{{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}"

; 生成器设置
Name "${APPNAME}"
OutFile "CodeNotebook-Setup-${APPVERSION}.exe"
InstallDir "$PROGRAMFILES64\${APPNAME}"
InstallDirRegKey HKLM "Software\${APPNAME}" "InstallPath"
RequestExecutionLevel admin
ShowInstDetails hide
ShowUnInstDetails hide

; 界面设置
!define MUI_ABORTWARNING
!define MUI_ICON "app.ico"  ; 如果有图标文件
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\modern-uninstall.ico"

; 安装页面
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE.txt"  ; 如果有许可证文件
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

; 卸载页面
!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

; 语言
!insertmacro MUI_LANGUAGE "SimpChinese"
!insertmacro MUI_LANGUAGE "English"

; 安装组件
Section "主程序" SecMain
    SectionIn RO

    SetOutPath "$INSTDIR"

    ; 安装主程序
    File "target\release\${APPEXE}"

    ; 创建开始菜单快捷方式
    CreateDirectory "$SMPROGRAMS\${APPNAME}"
    CreateShortCut "$SMPROGRAMS\${APPNAME}\${APPNAME}.lnk" "$INSTDIR\${APPEXE}" "" "$INSTDIR\${APPEXE}" 0
    CreateShortCut "$SMPROGRAMS\${APPNAME}\卸载.lnk" "$INSTDIR\Uninstall.exe"

    ; 创建桌面快捷方式
    CreateShortCut "$DESKTOP\${APPNAME}.lnk" "$INSTDIR\${APPEXE}"

    ; 注册表项
    WriteRegStr HKLM "Software\${APPNAME}" "InstallPath" "$INSTDIR"
    WriteRegStr HKLM "Software\${APPNAME}" "Version" "${APPVERSION}"

    ; 添加到程序和功能
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayName" "${APPNAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayVersion" "${APPVERSION}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "Publisher" "${APPPUBLISHER}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "UninstallString" "$INSTDIR\Uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayIcon" "$INSTDIR\${APPEXE}"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "NoModify" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "NoRepair" 1

    ; 创建卸载程序
    WriteUninstaller "$INSTDIR\Uninstall.exe"
SectionEnd

Section "右键菜单集成" SecContextMenu
    ; 为常见代码文件类型添加右键菜单
    DetailPrint "正在注册右键菜单..."

    ; 支持的文件扩展名
    !define EXTENSIONS ".rs,.c,.cpp,.h,.hpp,.py,.js,.ts,.jsx,.tsx,.java,.cs,.php,.rb,.go,.swift,.kt,.scala,.html,.css,.scss,.sass,.xml,.json,.yaml,.yml,.toml,.ini,.cfg,.conf,.sql,.sh,.bat,.cmd,.ps1,.reg,.inf,.log,.txt,.md,.markdown"

    ; 遍历扩展名并添加右键菜单
    Push $R0
    Push $R1

    StrCpy $R0 "${EXTENSIONS}"
    StrLen $R1 $R0
    IntOp $R1 $R1 + 1

    ${ForEach} $R1 1 $R1 + 1
        ${ExitFor}
    ${Next}

    ; 为每个扩展名创建右键菜单项
    DetailPrint "为代码文件添加右键菜单项..."

    ; 使用通用的"AllFileSystemObjects"来添加右键菜单
    WriteRegStr HKCR "*\shell\OpenWithCodeNotebook" "" "使用代码查看器打开"
    WriteRegStr HKCR "*\shell\OpenWithCodeNotebook\command" "" '"$INSTDIR\${APPEXE}" "%1"'

    ; 添加图标支持
    WriteRegStr HKCR "*\shell\OpenWithCodeNotebook" "Icon" "$INSTDIR\${APPEXE}"

    Pop $R1
    Pop $R0
SectionEnd

Section "文件关联" SecFileAssociation
    DetailPrint "正在设置文件关联..."

    ; 为常见的代码文件类型创建文件关联
    DetailPrint "创建代码文件类型..."

    ; Rust文件
    WriteRegStr HKCR ".rs" "" "RustSourceFile"
    WriteRegStr HKCR "RustSourceFile" "" "Rust 源文件"
    WriteRegStr HKCR "RustSourceFile\DefaultIcon" "" "$INSTDIR\${APPEXE},0"

    ; C/C++文件
    WriteRegStr HKCR ".c" "" "CSourceFile"
    WriteRegStr HKCR "CSourceFile" "" "C 源文件"
    WriteRegStr HKCR "CSourceFile\DefaultIcon" "" "$INSTDIR\${APPEXE},0"

    WriteRegStr HKCR ".cpp" "" "CppSourceFile"
    WriteRegStr HKCR "CppSourceFile" "" "C++ 源文件"
    WriteRegStr HKCR "CppSourceFile\DefaultIcon" "" "$INSTDIR\${APPEXE},0"

    WriteRegStr HKCR ".h" "" "CHeaderFile"
    WriteRegStr HKCR "CHeaderFile" "" "C/C++ 头文件"
    WriteRegStr HKCR "CHeaderFile\DefaultIcon" "" "$INSTDIR\${APPEXE},0"

    ; Python文件
    WriteRegStr HKCR ".py" "" "PythonSourceFile"
    WriteRegStr HKCR "PythonSourceFile" "" "Python 源文件"
    WriteRegStr HKCR "PythonSourceFile\DefaultIcon" "" "$INSTDIR\${APPEXE},0"

    ; JavaScript文件
    WriteRegStr HKCR ".js" "" "JavaScriptSourceFile"
    WriteRegStr HKCR "JavaScriptSourceFile" "" "JavaScript 源文件"
    WriteRegStr HKCR "JavaScriptSourceFile\DefaultIcon" "" "$INSTDIR\${APPEXE},0"

    ; TypeScript文件
    WriteRegStr HKCR ".ts" "" "TypeScriptSourceFile"
    WriteRegStr HKCR "TypeScriptSourceFile" "" "TypeScript 源文件"
    WriteRegStr HKCR "TypeScriptSourceFile\DefaultIcon" "" "$INSTDIR\${APPEXE},0"

    ; Java文件
    WriteRegStr HKCR ".java" "" "JavaSourceFile"
    WriteRegStr HKCR "JavaSourceFile" "" "Java 源文件"
    WriteRegStr HKCR "JavaSourceFile\DefaultIcon" "" "$INSTDIR\${APPEXE},0"

    ; 通用代码文件
    WriteRegStr HKCR ".txt" "" "TextFile"
    WriteRegStr HKCR "TextFile" "" "文本文件"
    WriteRegStr HKCR "TextFile\DefaultIcon" "" "$INSTDIR\${APPEXE},0"

    ; 为这些文件类型添加打开命令
    WriteRegStr HKCR "RustSourceFile\shell\open\command" "" '"$INSTDIR\${APPEXE}" "%1"'
    WriteRegStr HKCR "CSourceFile\shell\open\command" "" '"$INSTDIR\${APPEXE}" "%1"'
    WriteRegStr HKCR "CppSourceFile\shell\open\command" "" '"$INSTDIR\${APPEXE}" "%1"'
    WriteRegStr HKCR "CHeaderFile\shell\open\command" "" '"$INSTDIR\${APPEXE}" "%1"'
    WriteRegStr HKCR "PythonSourceFile\shell\open\command" "" '"$INSTDIR\${APPEXE}" "%1"'
    WriteRegStr HKCR "JavaScriptSourceFile\shell\open\command" "" '"$INSTDIR\${APPEXE}" "%1"'
    WriteRegStr HKCR "TypeScriptSourceFile\shell\open\command" "" '"$INSTDIR\${APPEXE}" "%1"'
    WriteRegStr HKCR "JavaSourceFile\shell\open\command" "" '"$INSTDIR\${APPEXE}" "%1"'
SectionEnd

; 组件描述
LangString DESC_SecMain ${LANG_SIMPCHINESE} "安装主程序文件"
LangString DESC_SecContextMenu ${LANG_SIMPCHINESE} "在文件右键菜单中添加'使用代码查看器打开'选项"
LangString DESC_SecFileAssociation ${LANG_SIMPCHINESE} "将程序设置为默认代码文件查看器"

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
    !insertmacro MUI_DESCRIPTION_TEXT ${SecMain} $(DESC_SecMain)
    !insertmacro MUI_DESCRIPTION_TEXT ${SecContextMenu} $(DESC_SecContextMenu)
    !insertmacro MUI_DESCRIPTION_TEXT ${SecFileAssociation} $(DESC_SecFileAssociation)
!insertmacro MUI_FUNCTION_DESCRIPTION_END

; 卸载程序
Section "Uninstall"
    ; 删除文件
    Delete "$INSTDIR\${APPEXE}"
    Delete "$INSTDIR\Uninstall.exe"

    ; 删除开始菜单快捷方式
    Delete "$SMPROGRAMS\${APPNAME}\*.*"
    RMDir "$SMPROGRAMS\${APPNAME}"

    ; 删除桌面快捷方式
    Delete "$DESKTOP\${APPNAME}.lnk"

    ; 删除注册表项
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}"
    DeleteRegKey HKLM "Software\${APPNAME}"

    ; 删除右键菜单
    DeleteRegKey HKCR "*\shell\OpenWithCodeNotebook"

    ; 删除文件关联
    DeleteRegKey HKCR ".rs"
    DeleteRegKey HKCR ".c"
    DeleteRegKey HKCR ".cpp"
    DeleteRegKey HKCR ".h"
    DeleteRegKey HKCR ".py"
    DeleteRegKey HKCR ".js"
    DeleteRegKey HKCR ".ts"
    DeleteRegKey HKCR ".java"

    DeleteRegKey HKCR "RustSourceFile"
    DeleteRegKey HKCR "CSourceFile"
    DeleteRegKey HKCR "CppSourceFile"
    DeleteRegKey HKCR "CHeaderFile"
    DeleteRegKey HKCR "PythonSourceFile"
    DeleteRegKey HKCR "JavaScriptSourceFile"
    DeleteRegKey HKCR "TypeScriptSourceFile"
    DeleteRegKey HKCR "JavaSourceFile"

    ; 删除安装目录（如果为空）
    RMDir "$INSTDIR"
SectionEnd

; 安装初始化
Function .onInit
    ; 检查Windows版本
    ${IfNot} ${AtLeastWin10}
        MessageBox MB_OK|MB_ICONSTOP "此程序需要Windows 10或更高版本。"
        Abort
    ${EndIf}

    ; 检查是否已安装
    ReadRegStr $R0 HKLM "Software\${APPNAME}" "InstallPath"
    StrCmp $R0 "" done

    MessageBox MB_YESNO|MB_ICONQUESTION "检测到程序已安装在 $R0。$\n$\n是否要卸载旧版本并继续安装？" IDYES uninst
    Abort

uninst:
    ClearErrors
    ExecWait '"$R0\Uninstall.exe" /S _?=$R0'

    IfErrors no_remove_uninstaller done
    no_remove_uninstaller:
done:
FunctionEnd