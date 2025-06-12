-- Get the head of a list (return 0 if empty)
headInt :: [Int] -> Int
headInt xs
  | length xs == 0 = 0
  | otherwise = sum (take 1 xs)

-- Get the tail of a list (drop first element)
tailInt :: [Int] -> [Int]
tailInt xs
  | length xs == 0 = []
  | otherwise = drop 1 xs

-- Check if a number is even
isEven :: Int -> Bool
isEven n
  | n == 0 = True
  | n == 1 = False
  | otherwise = isEven (n - 2)

-- Sum list elements
sumList :: [Int] -> Int
sumList xs
  | length xs == 0 = 0
  | otherwise = headInt xs + sumList (tailInt xs)

-- Double list elements
doubleList :: [Int] -> [Int]
doubleList xs
  | length xs == 0 = []
  | otherwise = (2 * headInt xs) : doubleList (tailInt xs)

-- Count evens
countEvens :: [Int] -> Int
countEvens xs
  | length xs == 0 = 0
  | isEven (headInt xs) = 1 + countEvens (tailInt xs)
  | otherwise = countEvens (tailInt xs)

-- Examples
example1 :: Int
example1 = sumList [4, 5, 6]

example2 :: [Int]
example2 = doubleList [1, 3, 5]

example3 :: Int
example3 = countEvens [2, 3, 4, 7]
