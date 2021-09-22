
import Data.Char
import Data.Maybe

generateHashtag :: String -> Maybe String
generateHashtag [] = Nothing 
generateHashtag str 
  | (length . strip $ str) == 0 = Nothing
  | length str > 140 = Nothing
  | otherwise = Just . ((:) '#') .strip . unwords  $ map (\x -> toUpper (head x) : (tail x)) $ words str

strip :: String -> String
strip str = filter (\x -> x /= ' ' && x /= '\t' && x /= '\n') str

