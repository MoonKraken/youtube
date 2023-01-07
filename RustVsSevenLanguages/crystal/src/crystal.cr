def get_sum(arr : Array(String))
  arr.map{ |elem| elem.to_i }.sum()
end

while true
  line = gets || "0"
  splits = line.split()
  puts get_sum(splits)
end
