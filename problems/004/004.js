function isPalindrome(n) {
    var reversed = 0;
    var tmp = n;

    while (tmp !== 0) {
        reversed = 10 * reversed + (tmp % 10);
        tmp = (tmp / 10) | 0;
    }

    return reversed === n;
}

function f() {
    var i, j, p;
    var max = 0;

    for (i = 100; i < 1000; i++) {
        for (j = 100; j < i; j++) {
            p = i * j;

            if (p > max && isPalindrome(p)) {
                max = p;
            }
        }
    }

    return max;
}

function toNs(ts) {
    return ts.sec * 1000000000 + ts.nsec;
}

var clock = require('posix-clock');

if (process.argv.length === 3) {
    var i;
    var iters = parseInt(process.argv[2], 10);

    var start = clock.gettime(clock.MONOTONIC);
    for (i = 0; i < iters; i++) {
        f();
    }
    var end = clock.gettime(clock.MONOTONIC);

    console.log(toNs(end) - toNs(start));
} else {
    console.log(f());
}
