defmodule SumModule do
    def io_loop() do
      IO.read(:stdio, :line)
      |> String.split(" ")
      |> SumModule.sum_fn
      |> IO.puts
      io_loop()
    end

    def sum_fn(strs) do
        case strs do
          [curr | rest] ->
            sum_fn(rest) +
            (String.trim(curr)
            |> String.to_integer)
          [] -> 0
        end
    end
end
