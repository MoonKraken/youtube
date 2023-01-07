function compute_sum(arr)
  local accum = 0
  for element in arr do
    accum = tonumber(element) + accum
  end
  return accum
end

while(true) do
  line = io.read()
  total = compute_sum(string.gmatch(line, "%S+"))
  print(tostring(total))
end
