<?php

$input = 'cxdnnyjw';

$index = 0;

$password = '';

while (strlen($password) < 8) {
    $hash = md5($input . $index);

    $firstFive = substr($hash, 0, 5);
    $sixth = substr($hash, 5, 1);

    if ($firstFive === '00000') {
        $password .= $sixth;
        echo $password . PHP_EOL;
    }

    $index += 1;
}
