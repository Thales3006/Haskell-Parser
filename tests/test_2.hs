
ident :: a -> a
ident n = n

addtolist :: a -> [a] -> [a]
addtolist x xs = x:xs 

data Shape = Circle Shape Int | Shapeless

makeCircle :: Shape-> Shape
makeCircle Shapeless = Shapeless
makeCircle (Circle n m) = Circle n m