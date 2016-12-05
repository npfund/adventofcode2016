<?php

$input = file(__DIR__ . '/input.txt', FILE_IGNORE_NEW_LINES);

$keypad = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];

$deltas = [
    'U' => [0, -1],
    'R' => [1, 0],
    'D' => [0, 1],
    'L' => [-1 , 0],
];

$current = [1, 1];
$code = '';

foreach ($input as $line) {
    $directions = str_split($line);

    foreach ($directions as $direction) {
        $delta = $deltas[$direction];

        $current[0] += $delta[0];
        $current[1] += $delta[1];

        if ($current[0] < 0) {
            $current[0] = 0;
        }

        if ($current[1] < 0) {
            $current[1] = 0;
        }

        if ($current[0] > 2) {
            $current[0] = 2;
        }

        if ($current[1] > 2) {
            $current[1] = 2;
        }
    }

    $code .= $keypad[$current[1]][$current[0]];
}

echo $code . PHP_EOL;
