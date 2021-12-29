param (
    [Parameter(Mandatory="true")][string]$TargetLink
)

$ErrorActionPreference = "Stop"
$HighPerformance = "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"
$scriptPath = $MyInvocation.MyCommand.Path

function Set-Powerplan($TargetPlan) {
    $CurrentPlan = $(powercfg -getactivescheme).split()[3]
    if ($CurrentPlan -ne $TargetPlan) {
        powercfg -setactive $TargetPlan
    }
    return $CurrentPlan
}

function New-Shortcut($TargetPath, $ShortcutPath) {
    $WScriptObj = New-Object -ComObject ("WScript.Shell")
    $shortcut = $WscriptObj.CreateShortcut("$ShortcutPath")
    $shortcut.TargetPath = "powershell.exe"
    $shortcut.IconLocation = $TargetPath
    $shortcut.Arguments = "-WindowStyle Hidden -File `"$scriptPath`" `"$TargetPath`""
    $shortcut.WorkingDirectory = Split-Path -Parent $TargetPath
    $shortcut.WindowStyle = 7 # Normal = 1, Maximized = 3, Minimized = 7
    $shortcut.Save()
}

$PreviousPlan = Set-Powerplan($HighPerformance)
$AmpSimProcess = Start-Process -FilePath "$TargetLink" -PassThru
$AmpSimProcess.PriorityClass = "High"
Wait-Process -InputObject $AmpSimProcess
Set-Powerplan($PreviousPlan)
