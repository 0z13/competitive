
valid :: String  -> String -> Bool
valid [] [] = True
valid [] x = False
valid (x:xs) q  
  | x == '(' || x == '[' || x == '{' = valid xs (x:q)
  | null q = False
  | x == ')' = if (head q) == '(' then valid xs (tail q) else False
  | x == '}' = if (head q) == '{' then valid xs (tail q) else False
  | x == ']' = if (head q) == '[' then valid xs (tail q) else False

