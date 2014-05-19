def step_sum(end_, step)
  e = (end_ - 1) / step

  step * e * (e + 1) / 2
end

def f
  end_ = 1000

  step_sum(end_, 3) + step_sum(end_, 5) - step_sum(end_, 15)
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
