<?php

$input = file(__DIR__ . '/input.txt', FILE_IGNORE_NEW_LINES);

foreach ($input as $line) {
    preg_match('/([a-z\-]+)([0-9]+)\[([a-z]+)\]/', $line, $matches);

    $name = trim(str_replace('-', ' ', $matches[1]));
    $sectorId = (int)$matches[2];
    $checksum = $matches[3];

    $letters = str_split(str_replace('-', '', $matches[1]));

    $count = [];
    foreach ($letters as $letter) {
        $count[$letter] = ($count[$letter] ?? 0) + 1;
    }

    uksort(
        $count,
        function ($lKey, $rKey) use ($count) {
            $sort = $count[$rKey] <=> $count[$lKey];

            if ($sort === 0) {
                $sort = $lKey <=> $rKey;
            }

            return $sort;
        }
    );

    $topFive = implode('', array_slice(array_keys($count), 0, 5));
    if ($topFive === $checksum) {
        echo decrypt($name, $sectorId) . ' ' . $sectorId . PHP_EOL;
    }
}

function decrypt($name, $sectorId) {
    $letters = str_split($name);
    $decrypted = '';

    foreach ($letters as $letter) {
        if ($letter !== ' ') {
            $decrypted .= chr(((ord($letter) - ord('a')) + $sectorId) % 26 + ord('a'));
        } else {
            $decrypted .= ' ';
        }
    }

    return $decrypted;
}
