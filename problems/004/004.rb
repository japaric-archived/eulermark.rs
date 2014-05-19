def palindrome?(n)
  reversed, tmp = 0, n

  while tmp != 0
    reversed = 10 * reversed + (tmp % 10)
    tmp /= 10
  end

  reversed == n
end

def f
  max = 0

  (100..1000).each do |i|
    (100..i).each do |j|
      p = i * j

      p > max && palindrome?(p) && max = p
    end
  end

  max
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
