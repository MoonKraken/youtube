module Main where

main :: IO ()
main = getLine >>= print . sum . map read . words >>= const main
