module Day1 where

increased_depths :: [Int] -> Int
increased_depths [] = 0
increased_depths [x] = 0
increased_depths (x : xs)
  | x < (head xs) = 1 + increased_depths (xs)
  | otherwise = increased_depths (xs)

three_sliding_window_sum :: [Int] -> [Int]
three_sliding_window_sum [] = []
three_sliding_window_sum [a] = []
three_sliding_window_sum [a, b] = []
three_sliding_window_sum [a, b, c] = [a + b + c]
three_sliding_window_sum list = sum (take 3 list) : three_sliding_window_sum (tail list)
