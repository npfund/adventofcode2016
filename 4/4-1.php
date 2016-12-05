<?php

$input = file(__DIR__ . '/input.txt', FILE_IGNORE_NEW_LINES);

$sum = 0;

foreach ($input as $line) {
    preg_match('/([a-z\-]+)([0-9]+)\[([a-z]+)\]/', $line, $matches);

    $name = str_replace('-', '', $matches[1]);
    $sectorId = (int) $matches[2];
    $checksum = $matches[3];

    $letters = str_split($name);

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
        $sum += $sectorId;
    }
}

echo $sum . PHP_EOL;
