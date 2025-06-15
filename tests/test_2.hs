
ident :: a -> a
ident n = n

addtolist :: [a] -> a
addtolist (x:xs) = x 

data Shape = Circle Shape Int | Shapeless

makeCircle :: Shape -> (Shape) 
makeCircle Shapeless = Shapeless
makeCircle (Circle n m) = Circle n (m+1)