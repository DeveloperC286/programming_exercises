import Day2
import System.Exit
import Test.HUnit

test_calculate_position :: Test
test_calculate_position = TestCase (assertEqual "" 150 (calculate_position [(Forward, 5), (Down, 5), (Forward, 8), (Up, 3), (Down, 8), (Forward, 2)]))

test_calculate_position_with_aim :: Test
test_calculate_position_with_aim = TestCase (assertEqual "" 900 (calculate_position_with_aim [(Forward, 5), (Down, 5), (Forward, 8), (Up, 3), (Down, 8), (Forward, 2)]))

tests :: Test
tests = TestList [TestLabel "test_calculate_position" test_calculate_position, TestLabel "test_calculate_position_with_aim" test_calculate_position_with_aim]

main = do
  count <- runTestTT tests
  if failures count > 0 then exitFailure else return ()
