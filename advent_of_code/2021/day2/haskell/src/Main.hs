module Main where

import Day2

main :: IO ()
main = do
  file_contents <- readFile "./input.txt"

  let commands :: [(Direction, Int)]
      commands = [(command, int) | line <- lines file_contents, let s = span (/= ' ') line, let command = read $ fst s, let int = read $ snd s]

      part_1_answer = calculatePosition commands
      part_2_answer = calculatePositionWithAim commands

  print part_1_answer
  print part_2_answer
