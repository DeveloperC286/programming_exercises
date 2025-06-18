module Day1 where

increasedDepths :: [Int] -> Int
increasedDepths [] = 0
increasedDepths [x] = 0
increasedDepths (x : xs)
  | x < head xs = 1 + increasedDepths xs
  | otherwise = increasedDepths xs

threeSlidingWindowSum :: [Int] -> [Int]
threeSlidingWindowSum [] = []
threeSlidingWindowSum [a] = []
threeSlidingWindowSum [a, b] = []
threeSlidingWindowSum [a, b, c] = [a + b + c]
threeSlidingWindowSum list = sum (take 3 list) : threeSlidingWindowSum (tail list)
