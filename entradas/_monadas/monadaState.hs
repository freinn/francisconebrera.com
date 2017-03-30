data State s a = State (s -> (a, s))

instance Functor (State s) where
  fmap f (State s) = State (\estado -> let (a, estado') = s estado
                                       in (f a, estado'))

runState :: State s a -> s -> (a, s)
runState (State s) s' = s s'

instance Applicative (State s) where
  pure x    = State $ \s0 -> (x, s0)
  fs <*> xs = State $ \s0 -> let (f, s1) = runState fs s0
                                 (x, s2) = runState xs s1
                             in  (f x, s2)

-- Para combinar computaciones con estado, queremos que el operador >>= (bind) devuelva
-- una nueva computación con estado que, dado un estado inicial,

-- 0. ejecute la computación de la izquierda
-- 1. pase su resultado al lado derecho.
-- 2. ejecute la computación de la derecha (creando una mónada que encapsula una función)
--    con el resultado de la computación de la izquierda y el estado producido por la computación de la izquierda
-- 3. devuelva 1) el resultado 2) el estado final de la computación creada

instance Monad (State s) where
  return x = State $ \s0 -> (x, s0)

  -- (>>=) :: M a -> (a -> M b) -> M b
  -- (>>=) :: (State s) a -> (a -> (State s) b) -> (State s) b
  (>>=) (State s) f = State $ \s0 -> let (a, s')  = s s0
                                     in runState (f a) s'

-- expression                 │ returns
-- ───────────────────────────┼────────────
-- runState (return "foo") 42 │ ("foo", 42)
-- runState (get)          42 │ (42,    42)
-- runState (put 43)       42 │ ((),    43)

get :: State s s
get = State $ \s -> (s, s)

put :: s -> State s ()
put s = State $ \_ -> ((), s)

encadenar :: Num s => State s String
encadenar = do
  s <- get
  put (s + 1)
  s' <- get
  put (s' * 2)
  return "operaciones completadas"

-- *Main> runState encadenar 45
-- ("foo",46)

encadenarSinAzucar :: Num s => State s String
encadenarSinAzucar = get >>= \s -> put (s+1) >> get
                           >>= \s' -> put (s' * 2) >> return "operaciones completadas"