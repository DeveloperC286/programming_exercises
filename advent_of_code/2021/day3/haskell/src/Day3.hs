module Day3 where

import Data.Char
import Data.List

toDecimal :: String -> Int
toDecimal = foldl' (\acc x -> acc * 2 + digitToInt x) 0

countNumberOfZeros :: String -> Int
countNumberOfZeros line = length $ filter (== '0') line

countNumberOfOnes :: String -> Int
countNumberOfOnes line = length $ filter (== '1') line

calculatePowerConsumption :: [String] -> Int
calculatePowerConsumption [] = 0
calculatePowerConsumption binaryNumbers = calculateEpsilonRate binaryNumbers * calculateGammaRate binaryNumbers

calculateGammaRate :: [String] -> Int
calculateGammaRate [] = 0
calculateGammaRate binaryNumbers = toDecimal binaryNumber
  where
    binaryNumber = map (\transposedBinaryNumber -> if countNumberOfZeros transposedBinaryNumber > countNumberOfOnes transposedBinaryNumber then '0' else '1') $ transpose binaryNumbers

calculateEpsilonRate :: [String] -> Int
calculateEpsilonRate [] = 0
calculateEpsilonRate binaryNumbers = 2 ^ length (head binaryNumbers) - (1 + calculateGammaRate binaryNumbers)

calculateLifeSupportRating :: [String] -> Int
calculateLifeSupportRating [] = 0
calculateLifeSupportRating binaryNumbers = calculateOxygenGeneratorRating binaryNumbers * calculateCO2ScrubberRating binaryNumbers

calculateOxygenGeneratorRating :: [String] -> Int
calculateOxygenGeneratorRating [] = 0
calculateOxygenGeneratorRating binaryNumbers = 0

calculateCO2ScrubberRating :: [String] -> Int
calculateCO2ScrubberRating [] = 0
calculateCO2ScrubberRating binaryNumbers = 0
