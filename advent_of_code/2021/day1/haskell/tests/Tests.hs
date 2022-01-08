import Control.Monad
import Day1
import System.Exit
import Test.HUnit

testIncreasedDepths :: Test
testIncreasedDepths = TestCase (assertEqual "" 7 (increasedDepths [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]))

testIncreasedDepthsSlidingWindow :: Test
testIncreasedDepthsSlidingWindow = TestCase (assertEqual "" 5 (increasedDepths (threeSlidingWindowSum [199, 200, 208, 210, 200, 207, 240, 269, 260, 263])))

tests :: Test
tests = TestList [TestLabel "testIncreasedDepths" testIncreasedDepths, TestLabel "testIncreasedDepthsSlidingWindow" testIncreasedDepthsSlidingWindow]

main = do
  count <- runTestTT tests
  Control.Monad.when (failures count > 0) exitFailure
