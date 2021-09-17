import Data.Char
solution :: String -> String
solution [] = []
solution (x:y:xs) = if isUpper y then x:' ':y:(solution xs)
                  else x:y:(solution xs)
solution (x:xs) = x:xs
