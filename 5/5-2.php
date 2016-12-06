<?php

$input = 'cxdnnyjw';

$index = 0;

$password = [];

while (count($password) < 8) {
    $hash = md5($input . $index);

    $firstFive = substr($hash, 0, 5);
    $position = intval(substr($hash, 5, 1), 16);

    if ($firstFive === '00000' && $position < 8 && !isset($password[$position])) {
        $char = substr($hash, 6, 1);

        $password[$position] = $char;
        echo implode('', $password) . PHP_EOL;
    }

    $index += 1;
}

ksort($password);

echo implode('', $password) . PHP_EOL;
