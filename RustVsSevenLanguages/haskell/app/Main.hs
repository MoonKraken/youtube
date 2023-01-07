module Main where

sum_fn l = sum $ map (\s->read s :: Integer) l
-- main :: IO ()
main = do
  line <- getLine
  putStrLn $ show $ sum_fn $ words line
  main
