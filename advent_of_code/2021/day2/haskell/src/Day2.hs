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

calculatePosition :: [(Direction, Int)] -> Int
calculatePosition [] = 0
calculatePosition commands = horizontal * abs (up - down)
  where
    horizontal = sum $ map snd $ filter ((== Forward) . fst) commands
    down = sum $ map snd $ filter ((== Down) . fst) commands
    up = sum $ map snd $ filter ((== Up) . fst) commands

calculatePositionWithAim :: [(Direction, Int)] -> Int
calculatePositionWithAim [] = 0
calculatePositionWithAim commands = horizontal * abs depth
  where
    horizontal = sum $ map snd $ filter ((== Forward) . fst) commands
    depth = sum $ map (\(aim, (command, value)) -> aim * value) $ filter (\(_, (command, _)) -> command == Forward) (zip (calculateAim commands) commands)

calculateAim :: [(Direction, Int)] -> [Int]
calculateAim [] = []
calculateAim commands = map snd $ scanl (\(discard, aim) (command, aim_delta) -> (command, applyToAim (aim, (command, aim_delta)))) (Up, 0) commands

applyToAim :: (Int, (Direction, Int)) -> Int
applyToAim (aim, (command, aim_delta))
  | command == Up = aim - aim_delta
  | command == Down = aim + aim_delta
  | otherwise = aim
