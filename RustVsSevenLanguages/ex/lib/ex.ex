defmodule SumModule do
  def io_loop() do
    IO.read(:stdio, :line)
    |> String.split()
    |> sum_fn(0)
    |> IO.puts()

    io_loop()
  end

  defp sum_fn([curr | rest], acc), do: sum_fn(rest, String.to_integer(curr) + acc)
  defp sum_fn([], acc), do: acc
end
