def f
  perimeter = 1000

  ((perimeter / 3 + 1)..(perimeter / 2)).each do |c|
    (((perimeter - c) / 2 + 1)..c).each do |b|
      a = perimeter - b - c

      return a * b * c if a * a + b * b == c * c
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
