function gcd(a, b) {
    if (a < b) {
        var t = a;
        a = b;
        b = t;
    }

    var r;
    while (b !== 0) {
        r = a % b;
        a = b;
        b = r;
    }

    return a;
}

function lcm(a, b) {
    return a * b / gcd(a, b);
}

function f() {
    var n = 2;

    var i;
    for (i = 3; i < 21; i++) {
        n = lcm(n, i);
    }

    return n;
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
