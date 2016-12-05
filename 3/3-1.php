<?php

$input = file(__DIR__ . '/input.txt', FILE_IGNORE_NEW_LINES);

$valid = 0;

foreach ($input as $line) {
    $line = preg_replace('/\s+/', ' ', $line);

    $sides = array_map(
        function (string $val) {
            return (int)$val;
        },
        explode(' ', ltrim($line))
    );

    if ((($sides[0] + $sides[1]) > $sides[2])
        && ($sides[1] + $sides[2] > $sides[0])
        && ($sides[2] + $sides[0] > $sides[1])) {
        $valid += 1;
    }
}

echo $valid . PHP_EOL;
