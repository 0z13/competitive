type Money = Int
data CanHe = NO | YES deriving (Show,Eq)

tickets :: [Money] -> CanHe
tickets xs= f xs 0


f :: [Money] -> Money -> CanHe 
f []     m =YES 
f (x:xs) m = case x of 
   25 -> f xs (m + 25) 
   50 -> if m >= 25 then f xs (m - 25) else NO
   75 -> if m >= 50 then f xs (m - 50) else NO
   100 -> if m >= 75 then f xs (m - 75) else NO
   _   -> NO
