function f() {
    var ans = 0;
    var curr = 1;
    var next = 2;
    var tmp;

    while (curr < 4000000) {
        if (curr % 2 === 0) {
            ans += curr;
        }

        tmp = next;
        next += curr;
        curr = tmp;
    }

    return ans;
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
