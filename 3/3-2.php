<?php

$input = file(__DIR__ . '/input.txt', FILE_IGNORE_NEW_LINES);

$buffer = [
    [],
    [],
    [],
];

$valid = 0;

foreach ($input as $line) {
    $line = preg_replace('/\s+/', ' ', $line);

    $partial = array_map(
        function (string $val) {
            return (int) $val;
        },
        explode(' ', ltrim($line))
    );

    $buffer[0][] = $partial[0];
    $buffer[1][] = $partial[1];
    $buffer[2][] = $partial[2];

    if (count($buffer[0]) === 3) {
        foreach ($buffer as $sides) {
            if (($sides[0] + $sides[1] > $sides[2])
                && ($sides[1] + $sides[2] > $sides[0])
                && ($sides[2] + $sides[0] > $sides[1])
            ) {
                $valid += 1;
            }
        }

        $buffer = [
            [],
            [],
            [],
        ];
    }
}

echo $valid . PHP_EOL;
