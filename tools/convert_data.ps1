$ErrorActionPreference = 'Stop'
$dataDir = 'C:\harpo\HarpoS7.Family0\Data'
$outFile = Join-Path $PSScriptRoot '..\src\legacy\family0\data.rs'

function Get-Uints($path) {
    $text = Get-Content -Raw -LiteralPath $path
    # The only hex tokens in these Data files are the array elements.
    $nums = [regex]::Matches($text, '0x[0-9a-fA-F]+') | ForEach-Object { $_.Value.ToLower() }
    return , $nums
}

function Emit-Typed($name, $ty, $nums, $suffix) {
    $sb = New-Object System.Text.StringBuilder
    [void]$sb.AppendLine("pub const ${name}: [$ty; $($nums.Count)] = [")
    $line = '    '
    foreach ($n in $nums) {
        $tok = "${n}${suffix}, "
        if (($line.Length + $tok.Length) -gt 98) { [void]$sb.AppendLine($line.TrimEnd()); $line = '    ' }
        $line += $tok
    }
    if ($line.Trim().Length -gt 0) { [void]$sb.AppendLine($line.TrimEnd()) }
    [void]$sb.AppendLine('];')
    [void]$sb.AppendLine('')
    return $sb.ToString()
}
function Emit-Array($name, $nums) { return (Emit-Typed $name 'u32' $nums 'u32') }

$t1 = Get-Uints (Join-Path $dataDir 'Transform1Data.cs')
$shared = Get-Uints (Join-Path $dataDir 'SharedData.cs')

$header = @'
// SPDX-License-Identifier: LGPL-3.0-or-later
// Copyright (C) 2026 s7commplus-rs contributors
//
// GENERATED constant tables from bonk-dev/HarpoS7 (MIT) HarpoS7.Family0/Data/*.cs
// by scratchpad/convert_data.ps1. See LICENSE-HarpoS7.

//! Read-only constant tables used by the Family-0 transforms.

/// `PreSeedTransform`'s fixed 3-dword suffix written at workBuffer[0xC2..0xC5].
pub const MAGIC_POSTFIX: [u32; 3] = [0x4f5b_b379, 0x90ba_725f, 0x36a4_d7bb];

'@

# Transform7Data: Data[256] bytes, then Indexes[498], then Counts[498] (all 0x tokens, in order).
$t7 = Get-Uints (Join-Path $dataDir 'Transform7Data.cs')
$t7data = $t7[0..255]
$t7indexes = $t7[256..753]
$t7counts = $t7[754..1251]

$nl = [Environment]::NewLine
$out = $header
$out += ('/// Transform12 opcode metadata (embedded resource; read as little-endian u32).' + $nl)
$out += ('pub static TRANSFORM12_METADATA: &[u8] = include_bytes!("transform12_metadata.bin");' + $nl + $nl)
$out += ('/// Transform12 big-integer constant pool (768 x 24-byte packed field elements).' + $nl)
$out += ('pub static BIGINT_DATA: &[u8] = include_bytes!("bigint_data.bin");' + $nl + $nl)
$out += ('/// Transform1Data.Data - Monolith9 base table for PreSeedTransform (192 dwords).' + $nl)
$out += (Emit-Array 'TRANSFORM1_DATA' $t1)
$out += ('/// SharedData.Data - used by KeyDerivationTransform / Transform13 (36 dwords).' + $nl)
$out += (Emit-Array 'SHARED_DATA' $shared)
$out += ('/// Transform7Data.Data - the 256-byte table (Data[0xD8..] is the base point G).' + $nl)
$out += (Emit-Typed 'TRANSFORM7_DATA' 'u8' $t7data 'u8')
$out += ('/// Transform7Data.Indexes - into Transform12 metadata, selected per prng comb-bit.' + $nl)
$out += (Emit-Typed 'TRANSFORM7_INDEXES' 'usize' $t7indexes 'usize')
$out += ('/// Transform7Data.Counts - Transform12 op counts, paired with Indexes.' + $nl)
$out += (Emit-Typed 'TRANSFORM7_COUNTS' 'usize' $t7counts 'usize')

[System.IO.File]::WriteAllText($outFile, $out)
Write-Output "T1=$($t1.Count) SHARED=$($shared.Count) T7DATA=$($t7data.Count) IDX=$($t7indexes.Count) CNT=$($t7counts.Count)"
