function f() {
    // PROBLEM SOLUTION GOES HERE
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
