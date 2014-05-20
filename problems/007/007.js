function f() {
    var map = [];
    var i = 0;
    var n = 10000;
    var q = 1;
    var p, x;

    while (true) {
        q += 1;

        p = map[q];

        if (p === undefined) {
            map[q * q] = q;

            if (i === n) {
                return q;
            }

            i += 1;
        } else {
            delete map[q];

            x = p + q;

            while (map[x] !== undefined) {
                x += p;
            }

            map[x] = p;
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
