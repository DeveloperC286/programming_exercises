import Control.Monad
import Day2
import System.Exit
import Test.HUnit

test_calculatePosition :: Test
test_calculatePosition = TestCase (assertEqual "" 150 (calculatePosition [(Forward, 5), (Down, 5), (Forward, 8), (Up, 3), (Down, 8), (Forward, 2)]))

test_calculatePositionWithAim :: Test
test_calculatePositionWithAim = TestCase (assertEqual "" 900 (calculatePositionWithAim [(Forward, 5), (Down, 5), (Forward, 8), (Up, 3), (Down, 8), (Forward, 2)]))

tests :: Test
tests = TestList [TestLabel "test_calculatePosition" test_calculatePosition, TestLabel "test_calculatePositionWithAim" test_calculatePositionWithAim]

main = do
  count <- runTestTT tests
  Control.Monad.when (failures count > 0) exitFailure
