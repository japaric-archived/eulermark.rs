function f() {
    var n = 600851475143;
    var factor = 2;

    while (true) {
        if (n % factor === 0) {
            n /= factor;
        } else {
            factor += 1;
        }

        if (factor * factor > n) {
            return n;
        } if (n === 1) {
            return factor;
        }
    }
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
