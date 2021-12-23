module Day2 where

data Direction
  = Forward
  | Down
  | Up
  deriving (Eq)

instance Show Direction where
  show Forward = "forward"
  show Down = "down"
  show Up = "up"

instance Read Direction where
  readsPrec _ s
    | s == show Forward = [(Forward, "")]
    | s == show Down = [(Down, "")]
    | s == show Up = [(Up, "")]
    | otherwise = []

calculate_position :: [(Direction, Int)] -> Int
calculate_position [] = 0
calculate_position commands = horizontal * abs (up - down)
  where
    horizontal = sum $ map (snd) $ filter ((== Forward) . fst) commands
    down = sum $ map (snd) $ filter ((== Down) . fst) commands
    up = sum $ map (snd) $ filter ((== Up) . fst) commands

calculate_position_with_aim :: [(Direction, Int)] -> Int
calculate_position_with_aim [] = 0
calculate_position_with_aim commands = horizontal * abs (depth)
  where
    horizontal = sum $ map (snd) $ filter ((== Forward) . fst) commands
    depth = sum $ map (\(aim, (command, value)) -> aim * value) $ filter (\(_, (command, _)) -> command == Forward) (zip (calculate_aim commands) commands)

calculate_aim :: [(Direction, Int)] -> [Int]
calculate_aim [] = []
calculate_aim commands = map (snd) $ scanl (\(discard, aim) (command, aim_delta) -> (command, apply_to_aim (aim, (command, aim_delta)))) (Up, 0) commands

apply_to_aim :: (Int, (Direction, Int)) -> Int
apply_to_aim (aim, (command, aim_delta))
  | command == Up = aim - aim_delta
  | command == Down = aim + aim_delta
  | otherwise = aim
