function f() {
    var perimeter = 1000;
    var a, b, c;

    for (c = ((perimeter / 3) | 0) + 1; c < (perimeter / 2) | 0; c++) {
        for (b = (((perimeter - c) / 2) | 0) + 1; b < c; b++) {
            a = perimeter - b - c;

            if (a * a + b * b === c * c) {
                return a * b * c;
            }
        }
    }

    return 0;
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
