module Main where

import Day3

main :: IO ()
main = do
  file_contents <- readFile "./input.txt"

  let binaryNumbers :: [String]
      binaryNumbers = lines file_contents

      part_1_answer = calculatePowerConsumption binaryNumbers

  print part_1_answer
