import Control.Monad
import Day3
import System.Exit
import Test.HUnit

testCalculatePowerConsumption :: Test
testCalculatePowerConsumption = TestCase (assertEqual "" 198 (calculatePowerConsumption ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]))

testCalculateEpsilonRate :: Test
testCalculateEpsilonRate = TestCase (assertEqual "" 9 (calculateEpsilonRate ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]))

testCalculateGammaRate :: Test
testCalculateGammaRate = TestCase (assertEqual "" 22 (calculateGammaRate ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]))

testCalculateLifeSupportRating :: Test
testCalculateLifeSupportRating = TestCase (assertEqual "" 230 (calculateLifeSupportRating ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]))

testCalculateOxygenGeneratorRating :: Test
testCalculateOxygenGeneratorRating = TestCase (assertEqual "" 23 (calculateOxygenGeneratorRating ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]))

testCalculateCO2ScrubberRating :: Test
testCalculateCO2ScrubberRating = TestCase (assertEqual "" 10 (calculateCO2ScrubberRating ["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"]))

tests :: Test
tests = TestList [TestLabel "testCalculatePowerConsumption" testCalculatePowerConsumption, TestLabel "testCalculateEpsilonRate" testCalculateEpsilonRate, TestLabel "testCalculateGammaRate" testCalculateGammaRate, TestLabel "testCalculateLifeSupportRating" testCalculateLifeSupportRating, TestLabel "testCalculateOxygenGeneratorRating" testCalculateOxygenGeneratorRating, TestLabel "testCalculateCO2ScrubberRating" testCalculateCO2ScrubberRating]

main = do
  count <- runTestTT tests
  Control.Monad.when (failures count > 0) exitFailure
