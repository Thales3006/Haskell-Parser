-- factorial function n!
fac:: Int -> Int
fac 0 = 1
fac n = n * fac (n-1)

-- add function, gets a number and add 1
add :: Int -> Int
add n = n+1

-- list constant
list :: [Int]
list = [3, -add 3+1]

-- fibonacci with guards
fib :: Int -> Int
fib n
    | n<1 = 0
    | n==1 = 1
    | otherwise = fib(n-1) + fib(n-2)