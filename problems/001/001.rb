def f
  ans = 0

  (0..999).each do |i|
    ans += i if i % 3 == 0 || i % 5 == 0
  end

  ans
end

case ARGV.length
when 0
  puts f
when 1
  iters = ARGV.first.to_i

  start = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  (1..iters).each do ||
    f
  end
  end_ = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  elapsed = ((end_ - start) * 1e9).to_i

  puts elapsed
end
