import Control.Monad
import Day2
import System.Exit
import Test.HUnit

testCalculatePosition :: Test
testCalculatePosition = TestCase (assertEqual "" 150 (calculatePosition [(Forward, 5), (Down, 5), (Forward, 8), (Up, 3), (Down, 8), (Forward, 2)]))

testCalculatePositionWithAim :: Test
testCalculatePositionWithAim = TestCase (assertEqual "" 900 (calculatePositionWithAim [(Forward, 5), (Down, 5), (Forward, 8), (Up, 3), (Down, 8), (Forward, 2)]))

tests :: Test
tests = TestList [TestLabel "testCalculatePosition" testCalculatePosition, TestLabel "testCalculatePositionWithAim" testCalculatePositionWithAim]

main = do
  count <- runTestTT tests
  Control.Monad.when (failures count > 0) exitFailure
