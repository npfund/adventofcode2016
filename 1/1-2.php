<?php

$input = file_get_contents(__DIR__ . '/input.txt');

$directions = explode(', ', $input);

$deltas = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
];

$current = [0, 0];
$delta = 0;
$visited = [];

foreach ($directions as $direction) {
    $turn = $direction[0];
    $steps = (int) substr($direction, 1);

    if ($turn === 'R') {
        $delta = ($delta + 1) % 4;
    } else {
        $delta = $delta - 1;
        if ($delta < 0) {
            $delta = 3;
        }
    }

    for ($i = 0; $i < $steps; $i++) {
        $current[0] = $current[0] + $deltas[$delta][0];
        $current[1] = $current[1] + $deltas[$delta][1];

        $index = $current[0] . ',' . $current[1];
        if (isset($visited[$index])) {
            break 2;
        } else {
            $visited[$index] = true;
        }
    }
}

echo abs($current[0]) + abs($current[1]);
