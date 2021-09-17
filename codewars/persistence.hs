-- Multiplicative persistence

mult :: Int -> Int -> Int
mult 0 x = x
mult n x = let nxt = x * (n `mod` 10) in
            mult (n `div` 10) nxt 


persistence :: Int -> Int
persistence n = count n 0
  where count n c = if n `div` 10 == 0 then c
                    else count (mult n 1) (c + 1)


   
