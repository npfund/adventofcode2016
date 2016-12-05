<?php

$input = file(__DIR__ . '/input.txt', FILE_IGNORE_NEW_LINES);

$keypad = [
    [null, null, 1, null, null],
    [null, 2, 3, 4, null],
    [5, 6, 7, 8, 9],
    [null, 'A', 'B', 'C', null],
    [null, null, 'D', null, null],
];

$deltas = [
    'U' => [0, -1],
    'R' => [1, 0],
    'D' => [0, 1],
    'L' => [-1, 0],
];

$current = [0, 2];
$code = '';

foreach ($input as $line) {
    $directions = str_split($line);

    foreach ($directions as $direction) {
        $destination = [];
        $delta = $deltas[$direction];

        $destination[0] = $current[0] + $delta[0];
        $destination[1] = $current[1] + $delta[1];

        if (isset($keypad[$destination[1]][$destination[0]])) {
            $current = $destination;
        }
    }

    $code .= $keypad[$current[1]][$current[0]];
}

echo $code . PHP_EOL;
