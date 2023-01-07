function sum_fn(splits::Vector{SubString{String}})
    sum(map(x -> begin
                parse(Int, x)
            end,
            splits))
end

while true
    line = readline()
    splits = split(line, " ")
    num = sum_fn(splits)
    println(num)
end
