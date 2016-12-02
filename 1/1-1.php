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

    $current[0] = $current[0] + ($deltas[$delta][0] * $steps);
    $current[1] = $current[1] + ($deltas[$delta][1] * $steps);
}

echo abs($current[0]) + abs($current[1]);
