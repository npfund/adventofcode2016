<?php

$input = file(__DIR__ . '/input.txt', FILE_IGNORE_NEW_LINES);

$columns = [];

foreach ($input as $line) {
    $chars = str_split($line);

    foreach ($chars as $index => $char) {
        if (!isset($columns[$index])) {
            $columns[$index] = [];
        }

        $columns[$index][$char] = ($columns[$index][$char] ?? 0) + 1;
    }
}

foreach ($columns as $column) {
    asort($column);
    echo key($column);
}
