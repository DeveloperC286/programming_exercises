module Main (main) where

import Day1

main :: IO ()
main = do
  file_contents <- readFile "./input.txt"

  let depths :: [Int]
      depths = map read $ lines file_contents

      part_1_answer = increasedDepths depths
      part_2_answer = increasedDepths (threeSlidingWindowSum depths)

  print part_1_answer
  print part_2_answer
