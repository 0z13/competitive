import Data.List
longest :: [Char] -> [Char] -> [Char]
longest s1 s2 = nub . sort $ s1 ++ s2 
