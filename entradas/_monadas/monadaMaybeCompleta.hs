import Prelude hiding (Maybe, Nothing, Just)

-- Preliminares

sumar :: Int -> Int -> Int
sumar x y = x + y

sumar10 :: Int -> Int
sumar10 y = sumar 10 y

dividirEntreDos :: Int -> Maybe Int
dividirEntreDos x = flip division 2 x

-- *Main> dividirEntreDos 18
-- Just 9

{-
*Main> 234 `div` 0
*** Exception: divide by zero
-}

data Maybe a = Nothing
             | Just a
             deriving Show

instance Functor Maybe where
  fmap f (Just x) = Just (f x)
  fmap _ Nothing  = Nothing

instance Applicative Maybe where
  pure                  = Just
  (Just f) <*> (Just x) = Just (f x)
  _        <*> _        = Nothing

instance Monad Maybe where
  return x         = (Just x)
  
  (>>=) (Just x) f = f x     -- la computación se producirá normalmente (aunque puede que dé un Nothing)
  (>>=) Nothing  f = Nothing -- la computación no se puede realizar y se propaga el resultado

division :: Int -> Int -> Maybe Int
division _ 0 = Nothing
division x y = Just (x `div` y)

-- composición de funciones (.) :: (a -> b) -> (b -> c) -> (a -> c)

-- (Int -> Maybe Int) -> (Int -> Maybe Int) -> (Int -> Maybe Int)

-- flip :: (a -> b -> c) -> b -> a -> c
-- flip f x y =  f y x

dividirEntreCero :: Int -> Maybe Int
dividirEntreCero x = flip division 0 x

sucesionDeDivisiones = (Just 128) >>= dividirEntreDos >>= dividirEntreDos

sucesionDeDivisionesConCero = (Just 128) >>= dividirEntreDos >>= dividirEntreCero >>= dividirEntreDos

sucesionDeDivisionesDo = do
  numeroInicial <- Just 128
  resultado1 <- dividirEntreDos numeroInicial
  dividirEntreDos resultado1