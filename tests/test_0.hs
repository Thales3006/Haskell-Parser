-- factorial function n!
fac:: Int -> Int
fac 0 = 1
fac n = n * fac (n-1)

-- add function, gets a number and add 1
add :: Int -> Int
add n = n+1
list = [3, add 3+1]