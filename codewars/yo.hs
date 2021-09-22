{-# LANGUAGE NamedFieldPuns #-}
{-# LANGUAGE RecordWildCards #-}

{-
RecordWildCards allows you to write things like C{..} which is equivalent to the pattern: C{x=x, y=y, name=name}, i.e. it matches all fields and you now have in scope x with the value matched for the x field etc.

NamedFieldPuns allows you to write C{name} to be equivalent to C{name=name}, so that name is now in scope and contains the value matched for the name field.
 Constructor are capitalized 
fields are lowercase.
-}
data Car = MkCar { model :: String
                 , owner :: String
                 , year  :: Int 
                 }
  deriving (Show)

xmple :: Car
xmple = MkCar {model="Ford", owner="Maks", year=2013}

changeYear :: Car -> Int -> Car
changeYear MkCar{model=mod, owner=own, year=n} x = MkCar {model=mod, owner=own, year=x}

modelChange :: Car -> String -> Car
modelChange MkCar{..} s = MkCar{model=s, owner=owner, year=year}


data MyMaybe a = 
