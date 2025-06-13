-- Get the head of a list (return 0 if empty)
headInt :: [Int] -> Int
headInt xs
  | null xs = 0
  | otherwise = sum (take 1 xs)

-- Get the tail of a list (drop first element)
tailInt :: [Int] -> [Int]
tailInt xs
  | null xs = []
  | otherwise = drop 1 xs

-- Check if a number is even
isEven :: Int -> Bool
isEven n
  | n == 0 = True
  | n == 1 = False
  | otherwise = isEven (n - 2)

ifNull :: [a] -> b -> b -> b
ifNull xs vAlt vOk
  | null xs = vAlt
  | otherwise      = vOk

-- Sum list elements
sumList :: [Int] -> Int
sumList xs = ifNull xs 0 $ head xs + sumList (tailInt xs)

-- Double list elements
doubleList :: [Int] -> [Int]
doubleList xs = ifNull xs [] $ (2 ^ 1 * headInt xs) : doubleList (tailInt xs)

-- Count evens
countEvens :: [Int] -> Int
countEvens xs = ifNull xs 0 $ fromEnum (isEven $ xs !! 1) + countEvens (tailInt xs)

-- Examples
example1 :: Int
example1 = sumList [4, 5, 6]

example2 :: [Int]
example2 = doubleList [1, 3, 5]

example3 :: Int
example3 = countEvens [2, 3, 4, 7]
