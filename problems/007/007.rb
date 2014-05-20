def f
  i, map, n, q = 0, {}, 10_000, 1

  loop do
    q += 1

    p = map.delete(q)

    if p.nil?
      map[q * q] = q

      if i == n
        return q
      else
        i += 1
      end
    else
      x = p + q

      x += p while map.key?(x)

      map[x] = p
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
