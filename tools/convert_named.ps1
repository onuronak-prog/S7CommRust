$ErrorActionPreference = 'Stop'
$mono = 'C:\harpo\HarpoS7.Family0\Monoliths'
$outDir = Join-Path $PSScriptRoot '..\src\legacy\family0\monolith'

# Extract a method body (between the '{' after the signature and its matching '}').
function Get-MethodBody($text, $sigRegex) {
    $m = [regex]::Match($text, $sigRegex)
    if (-not $m.Success) { throw "sig not found: $sigRegex" }
    $i = $text.IndexOf('{', $m.Index)
    $depth = 0
    for ($j = $i; $j -lt $text.Length; $j++) {
        $ch = $text[$j]
        if ($ch -eq '{') { $depth++ }
        elseif ($ch -eq '}') { $depth--; if ($depth -eq 0) { return $text.Substring($i + 1, $j - $i - 1) } }
    }
    throw "unbalanced braces"
}

function Xform($t) {
    $t = $t -replace '~', '!'
    $t = $t -replace '\*\s*2(?![0-9a-fA-Fx])', '<< 1'
    return $t
}

# Convert a straight-line named-local Execute method body to Rust statement lines.
# Returns @{ src=<name>; dst=<name>; decls=@(); stmts=@() }
function Convert-Straight($body) {
    $srcName = ([regex]::Match($body, 'var (\w+) = MemoryMarshal\.Cast<byte, uint>\(source\)')).Groups[1].Value
    $dstName = ([regex]::Match($body, 'var (\w+) = MemoryMarshal\.Cast<byte, uint>\(destination\)')).Groups[1].Value
    $decls = [System.Collections.Generic.List[string]]::new()
    $seen = @{}
    foreach ($d in [regex]::Matches($body, '\buint (\w+)')) {
        $n = $d.Groups[1].Value
        if (-not $seen.ContainsKey($n)) { $seen[$n] = $true; $decls.Add($n) | Out-Null }
    }
    $stmts = [System.Collections.Generic.List[string]]::new()
    $retExpr = $null
    foreach ($c in ($body -split ';')) {
        $t = ($c -replace '\s+', ' ').Trim()
        $t = $t -replace '^[{}]\s*', ''                      # strip a leading brace (e.g. '} var ...')
        if ($t.Length -eq 0) { continue }
        if ($t -match '^var ') { continue }
        if ($t -match '^BufferLengthException') { continue }
        if ($t -match '^if ' -or $t -match '^throw' -or $t -match '^else') { continue }
        if ($t -match '^uint \w+$') { continue }             # pure declaration
        if ($t -match '^return (.+)$') { $retExpr = $Matches[1].Trim(); continue }
        $t = $t -replace '^uint ', ''                         # declare+assign -> assign
        if ($t -notmatch '=') { continue }                    # safety: only assignments
        $stmts.Add('    ' + (Xform $t) + ';') | Out-Null
    }
    return @{ src = $srcName; dst = $dstName; decls = $decls; stmts = $stmts; retExpr = $retExpr }
}

$header = @'
// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// GENERATED -- do not edit by hand. Mechanically transcribed from bonk-dev/HarpoS7
// (MIT) HarpoS7.Family0/Monoliths/MONO.cs by scratchpad/convert_named.ps1: a
// straight-line named-local circuit (only ^ & ~(->!) >> << *2(-><<1)). Validated
// byte-for-byte against the MONO-src/dst golden vector. See LICENSE-HarpoS7.
#![allow(clippy::all)]
#![allow(non_snake_case, unused_parens, unused_assignments, unused_variables, unused_mut)]

'@

# monolith index -> (src dwords, dst dwords)
$sizes = @{ 1 = @(18, 18); 2 = @(18, 5); 3 = @(42, 36); 4 = @(36, 18); 5 = @(54, 12);
    6 = @(54, 36); 7 = @(24, 36); 8 = @(18, 15); 11 = @(30, 5) }

foreach ($idx in @(1, 2, 3, 4, 5, 6, 7, 8, 11)) {
    $text = Get-Content -Raw (Join-Path $mono "Monolith$idx.cs")
    $text = [regex]::Replace($text, '/\*.*?\*/', '', [System.Text.RegularExpressions.RegexOptions]::Singleline)
    $text = [regex]::Replace($text, '//[^\r\n]*', '')
    $body = Get-MethodBody $text 'public static (void|uint) Execute\s*\('
    $r = Convert-Straight $body
    $ret = if ($r.retExpr) { ' -> u32' } else { '' }

    $sb = New-Object System.Text.StringBuilder
    [void]$sb.Append(($header -replace 'MONO', "Monolith$idx"))
    [void]$sb.AppendLine("pub fn execute(destination: &mut [u8], source: &[u8])$ret {")
    [void]$sb.AppendLine("    let $($r.src): Vec<u32> = source")
    [void]$sb.AppendLine("        .chunks_exact(4)")
    [void]$sb.AppendLine("        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))")
    [void]$sb.AppendLine("        .collect();")
    [void]$sb.AppendLine("    let mut $($r.dst) = [0u32; $($sizes[$idx][1])];")
    foreach ($d in $r.decls) { [void]$sb.AppendLine("    let mut ${d}: u32 = 0;") }
    foreach ($s in $r.stmts) { [void]$sb.AppendLine($s) }
    [void]$sb.AppendLine("    for (i, &w) in $($r.dst).iter().enumerate() {")
    [void]$sb.AppendLine("        destination[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());")
    [void]$sb.AppendLine("    }")
    if ($r.retExpr) { [void]$sb.AppendLine("    $(Xform $r.retExpr)") }
    [void]$sb.AppendLine("}")

    # Monolith1 also exposes Loop: iterate Execute (copying dst[..0x48] back to src) until nonzero.
    if ($idx -eq 1) {
        [void]$sb.AppendLine("")
        [void]$sb.AppendLine("/// `Monolith1.Loop`: run `execute` until it returns nonzero, feeding its output back in.")
        [void]$sb.AppendLine("pub fn loop_normalize(destination: &mut [u8], source: &mut [u8]) {")
        [void]$sb.AppendLine("    let mut r = execute(destination, source);")
        [void]$sb.AppendLine("    while r == 0 {")
        [void]$sb.AppendLine("        let copy: [u8; 0x48] = destination[..0x48].try_into().unwrap();")
        [void]$sb.AppendLine("        source[..0x48].copy_from_slice(&copy);")
        [void]$sb.AppendLine("        r = execute(destination, source);")
        [void]$sb.AppendLine("    }")
        [void]$sb.AppendLine("}")
    }

    $file = Join-Path $outDir "m$idx.rs"
    [System.IO.File]::WriteAllText($file, $sb.ToString())
    $lc = (Get-Content $file | Measure-Object -Line).Lines
    Write-Output "wrote m$idx.rs ($lc lines, $($r.decls.Count) locals, ret=$([bool]$r.retExpr), src=$($r.src) dst=$($r.dst))"
}
