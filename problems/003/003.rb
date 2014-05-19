def f
  n, factor = 600_851_475_143, 2

  loop do
    if n % factor == 0
      n /= factor
    else
      factor += 1
    end

    if factor * factor > n
      return n
    elsif n == 1
      return factor
    end
  end
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
