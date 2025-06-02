fac1:: Int -> Int
fac1 0 = 1
fac1 n = n * fac (n-1)

fac2:: Int -> Int
fac2 n 
    | n==0 = 1
    | otherwise = n * fac   (n-1)