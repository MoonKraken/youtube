defmodule ExTest do
  use ExUnit.Case
  doctest Ex

  test "greets the world" do
    assert Ex.hello() == :world
  end
end
