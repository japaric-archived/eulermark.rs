def f
  sum, sum_of_squares = 0, 0

  (1..100).each do |i|
    sum += i
    sum_of_squares += i * i
  end

  squared_sum = sum * sum

  squared_sum - sum_of_squares
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
