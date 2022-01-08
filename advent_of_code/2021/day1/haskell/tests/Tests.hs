import Day1
import System.Exit
import Test.HUnit

test_increased_depths :: Test
test_increased_depths = TestCase (assertEqual "" 7 (increased_depths [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]))

test_increased_depths_sliding_window :: Test
test_increased_depths_sliding_window = TestCase (assertEqual "" 5 (increased_depths (three_sliding_window_sum [199, 200, 208, 210, 200, 207, 240, 269, 260, 263])))

tests :: Test
tests = TestList [TestLabel "test_increased_depths" test_increased_depths, TestLabel "test_increased_depths_sliding_window" test_increased_depths_sliding_window]

main = do
  count <- runTestTT tests
  if failures count > 0 then exitFailure else return ()
