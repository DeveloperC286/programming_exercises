module Main (main) where

import Day1

main :: IO ()
main = do
  file_contents <- readFile "./input.txt"

  let depths :: [Int]
      depths = map read $ lines file_contents

      part_1_answer = increased_depths depths
      part_2_answer = increased_depths (three_sliding_window_sum depths)

  putStrLn $ show part_1_answer
  putStrLn $ show part_2_answer
