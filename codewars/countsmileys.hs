smileys = [":)", ":D", ";D", ";)", ":-D", ":-)", ";-D", ";-)",
            ":~)", ";~)", ";~D", ":~D"]

countSmileys :: [String] -> Int
countSmileys = foldr (\x acc -> if x `elem` smileys then acc + 1 else acc) 0
