
f :: Int -> Int -> Int -> Int
f 0 acc power = acc
f x acc power = f (x `div` 10) (acc + ((x `mod` 10)^power)) (power  - 1)


digitLength :: Int -> Int -> Int
digitLength 0 acc = acc
digitLength n acc = digitLength (n `div` 10) (acc + 1)

sumDigPow :: Int -> Int -> [Int]
sumDigPow a b = filter (\x -> (f x 0 (digitLength x 0) == x)) [a..b]
