import Data.List
findOdd :: [Int] -> Int
findOdd xs = (head . head) . (filter (\x -> (length x) `mod` 2 /= 0)) . group . sort $ xs
