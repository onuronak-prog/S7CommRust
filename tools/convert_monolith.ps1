$ErrorActionPreference = 'Stop'
$mono = 'C:\harpo\HarpoS7.Family0\Monoliths'
$outDir = Join-Path $PSScriptRoot '..\src\legacy\family0\monolith'
New-Item -ItemType Directory -Force $outDir | Out-Null

function Convert-Part($path) {
    $text = Get-Content -Raw -LiteralPath $path
    $text = [regex]::Replace($text, '/\*.*?\*/', '', [System.Text.RegularExpressions.RegexOptions]::Singleline)
    $text = [regex]::Replace($text, '//[^\r\n]*', '')
    $lines = New-Object System.Collections.Generic.List[string]
    foreach ($c in ($text -split ';')) {
        $t = ($c -replace '\s+', ' ').Trim()
        # Strip any leading preamble up to the last '{' (file-scoped namespace merges
        # the class/method decl AND the first statement into one ';'-chunk).
        $t = $t -replace '^.*\{\s*', ''
        if ($t -match '^(locals|dst)\[') {
            $t = $t -replace '~', '!'
            $t = $t -replace '\*\s*2(?![0-9a-fA-Fx])', '<< 1'
            $lines.Add('    ' + $t + ';') | Out-Null
        }
    }
    return ,$lines
}

$header = @'
// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// GENERATED — do not edit by hand. Mechanically transcribed from
// bonk-dev/HarpoS7 (MIT) HarpoS7.Family0/Monoliths/{0}/Part*.cs by
// scratchpad/convert_monolith.ps1: a straight-line bit-sliced boolean circuit
// (only ^ & ~(->!) >> << *2(-><<1)). Validated byte-for-byte against the
// monolith{1}-src/dst golden vectors. See LICENSE-HarpoS7.
#![allow(clippy::all)]
#![allow(unused_parens)]

'@

function Build-Monolith($name, $dir, $numParts, $localsSize, $srcParts, $dstU32, $srcName, $tag) {
    $sb = New-Object System.Text.StringBuilder
    [void]$sb.Append(($header -f $dir, $tag))
    for ($p = 1; $p -le $numParts; $p++) {
        $body = Convert-Part (Join-Path $mono "$dir\Part$p.cs")
        $sig = "fn part$p("
        $params = @()
        if ($srcParts -contains $p) { $params += 'src: &[u32]' }
        if ($p -eq $numParts) { $params += 'dst: &mut [u32]' }
        $params += 'locals: &mut [u32]'
        $sig += ($params -join ', ') + ') {'
        [void]$sb.AppendLine($sig)
        foreach ($l in $body) { [void]$sb.AppendLine($l) }
        [void]$sb.AppendLine('}')
        [void]$sb.AppendLine('')
    }
    # execute wrapper
    [void]$sb.AppendLine("pub fn execute(destination: &mut [u8], source: &[u8]) {")
    [void]$sb.AppendLine("    let src: Vec<u32> = source")
    [void]$sb.AppendLine("        .chunks_exact(4)")
    [void]$sb.AppendLine("        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))")
    [void]$sb.AppendLine("        .collect();")
    [void]$sb.AppendLine("    let mut locals = [0u32; $localsSize];")
    for ($p = 1; $p -le $numParts; $p++) {
        $args = @()
        if ($srcParts -contains $p) { $args += '&src' }
        if ($p -eq $numParts) { $args += '&mut dst' }
        $args += '&mut locals'
        if ($p -eq $numParts) {
            [void]$sb.AppendLine("    let mut dst = [0u32; $dstU32];")
        }
        [void]$sb.AppendLine("    part$p(" + ($args -join ', ') + ");")
    }
    [void]$sb.AppendLine("    for (i, &w) in dst.iter().enumerate() {")
    [void]$sb.AppendLine("        destination[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());")
    [void]$sb.AppendLine("    }")
    [void]$sb.AppendLine("}")
    $file = Join-Path $outDir "$name.rs"
    [System.IO.File]::WriteAllText($file, $sb.ToString())
    $lc = (Get-Content $file | Measure-Object -Line).Lines
    Write-Output "wrote $file ($lc lines)"
}

# Ten: Part1 reads src; Part3 writes dst. locals=301, dst=192 u32.
Build-Monolith 'ten' 'Ten' 3 301 @(1) 192 'source' '10'
# Nine: Part1 & Part2 read src; Part11 writes dst. locals=831, dst=6 u32.
Build-Monolith 'nine' 'Nine' 11 831 @(1,2) 6 'source' '9'
