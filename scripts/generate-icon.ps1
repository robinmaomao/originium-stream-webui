Add-Type -AssemblyName System.Drawing

$size = 1024
$bitmap = New-Object System.Drawing.Bitmap $size, $size
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias

$background = [System.Drawing.Color]::FromArgb(255, 11, 11, 15)
$graphics.Clear($background)

$rect = New-Object System.Drawing.Rectangle 0, 0, $size, $size
$gradient = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
    $rect,
    [System.Drawing.Color]::FromArgb(255, 124, 77, 255),
    [System.Drawing.Color]::FromArgb(255, 0, 229, 255),
    45
)

$outer = @(
    (New-Object System.Drawing.Point 512, 120),
    (New-Object System.Drawing.Point 864, 512),
    (New-Object System.Drawing.Point 512, 904),
    (New-Object System.Drawing.Point 160, 512)
)
$graphics.FillPolygon($gradient, $outer)

$inner = @(
    (New-Object System.Drawing.Point 512, 320),
    (New-Object System.Drawing.Point 676, 512),
    (New-Object System.Drawing.Point 512, 704),
    (New-Object System.Drawing.Point 348, 512)
)
$darkBrush = New-Object System.Drawing.SolidBrush $background
$graphics.FillPolygon($darkBrush, $inner)

$output = Join-Path $PSScriptRoot '..\apps\desktop\src-tauri\icons\source.png'
$output = [System.IO.Path]::GetFullPath($output)
$bitmap.Save($output, [System.Drawing.Imaging.ImageFormat]::Png)

$graphics.Dispose()
$bitmap.Dispose()
$gradient.Dispose()
$darkBrush.Dispose()

Write-Output "Icon source generated: $output"
