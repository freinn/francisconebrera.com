import Control.Monad
import System.Random
import qualified Control.Monad.Trans.State as State

rollDiceIO :: IO (Int, Int)
rollDiceIO = liftM2 (,) (randomRIO (1,6)) (randomRIO (1,6))

-- > :t randomRIO 
-- randomRIO :: Random a => (a, a) -> IO a

rollNDiceIO :: Int -> IO [Int]
rollNDiceIO n = mapM randomRIO (replicate n (1,6))

{-
La anotación de tipos es necesaria porque la función random
puede producir valores de muchos tipos

GHCi> :m System.Random
GHCi> let generator = mkStdGen 0           -- "0" is our seed
GHCi> :t generator
generator :: StdGen
GHCi> generator
1 1
GHCi> :t random
random :: (RandomGen g, Random a) => g -> (a, g)
GHCi> random generator :: (Int, StdGen)
(2092838931,1601120196 1655838864)

El siguiente código no vale porque siempre dará lo mismo

GHCi> let randInt = fst . random $ generator :: Int
GHCi> randInt
2092838931

Hay que asociar a un nombre el generador producido, y luego
usarlo para generar otro número

GHCi> let (randInt, generator') = random generator :: (Int, StdGen)
GHCi> randInt                            -- Same value
2092838931
GHCi> random generator' :: (Int, StdGen) -- Using new generator' returned from “random generator”
(-2143208520,439883729 1872071452)
-}

clumsyRollDice :: (Int, Int)
clumsyRollDice = (n, m)
  where
  (n, g) = randomR (1, 6) (mkStdGen 0)
  (m, _) = randomR (1, 6) g

rollDice :: StdGen -> ((Int, Int), StdGen)
rollDice gen = let (n, gen') = randomR (1, 6) gen
                   (m, gen'') = randomR (1,6) gen'
               in ((n, m), gen'')

-- la mónada estado no guarda estado, sino un procesador del estado
-- newtype State s a = State { runState :: s -> (a, s) }

-- se usa newtype (parecido a data) porque type no nos permite crear instancias

-- state :: (s -> (a, s)) -> State s a

-- instance Monad (State s) where
  -- return :: a -> State s a
  -- return x = state ( \ st -> (x, st) )

  -- (>>=) :: State s a -> (a -> State s b) -> State s b
  -- pr >>= k = state $ \ st ->
  --     let (x, st') = runState pr st -- Running the first processor on st.
  --     in runState (k x) st'       -- Running the second processor on st'.

{-
One detail in the implementation is how runState is used to undo the State wrapping,
so that we can reach the function that will be applied to the states. The type of
runState pr, for instance, is s -> (s, a).
-}

-- pone el estado de la mónada
put :: s -> State.State s ()
put newState = State.state $ \_ -> ((), newState)

get :: State.State s s
get = State.state $ \s0 -> (s0, s0)

-- ejecuta la computación y nos da el resultado
evalState :: State.State s a -> s -> a
evalState monada estadoAProcesar = fst $ State.runState monada estadoAProcesar

-- ejecuta la computación y nos da el estado
execState :: State.State s a -> s -> s
execState pr st = snd (State.runState pr st)

-- dados y estado

{-
the type of randomR is:

-- The StdGen type we are using is an instance of RandomGen.
randomR :: (Random a, RandomGen g) => (a, a) -> g -> (a, g)

Doesn't it look familiar? If we assume a is Int and g is StdGen it becomes:

randomR (1, 6) :: StdGen -> (Int, StdGen)
-}

-- interesa que se cree la computación, no ejecutarla
rollDice' :: State.State StdGen Int
rollDice' = State.state $ randomR (1, 6)

rollDie :: State.State StdGen Int
rollDie = do generator <- get
             let (value, newGenerator) = randomR (1,6) generator
             put newGenerator
             return value

-- se pasan el estado de uno a otro
rollDice'' :: State.State StdGen (Int, Int)
rollDice'' = liftM2 (,) rollDie rollDie

-- liftM2 f m1 m2          = do x1 <- m1
--                              x2 <- m2
--                              return (f x1 x2)

-- ejercicio 1

-- replicateM :: Monad m => Int -> m a -> m [a] Source
-- replicateM n act performs the action n times, gathering the results.

rollNDice :: Int -> State.State StdGen [Int]
rollNDice n = replicateM n tirarDados
  where tirarDados = State.state $ randomR (1, 6)

-- ejercicio 2
-- Cuando tenemos una monad y no sabemos cómo hacer la instancia de
-- Functor, por ejemplo, vemos que una ley dice que fmap = liftM en
-- una mónada bien creada (proper) y vamos desarrollando ese liftM
-- con su definición hasta llegar a la de fmap

data Estado s a = Estado (s -> (a, s))

estado :: (s -> (a, s)) -> Estado s a
estado = Estado

ejecutarEstado :: Estado s a -> s -> (a, s)
ejecutarEstado (Estado s) s0 = s s0

instance Functor (Estado s) where
  fmap f pr = estado $ \ st ->
    let (x, st') = ejecutarEstado pr st
    in (f x, st')

-- ejercicio 3

modify :: (s -> s) -> State.State s ()
modify f = State.state $ \s0 -> ((), f s0)

gets :: (s -> a) -> State.State s a
gets f = State.state $ \s0 -> (f s0, s0)

getRandom :: Random a => State.State StdGen a
getRandom = State.state random

-- GHCi> evalState getRandom (mkStdGen 0) :: Bool
-- True
-- GHCi> evalState getRandom (mkStdGen 0) :: Char
-- '\64685'
-- GHCi> evalState getRandom (mkStdGen 0) :: Double
-- 0.9872770354820595
-- GHCi> evalState getRandom (mkStdGen 0) :: Integer
-- 2092838931

someTypes :: State.State StdGen (Int, Float, Char)
someTypes = liftM3 (,,) getRandom getRandom getRandom

allTypes :: State.State StdGen (Int, Float, Char, Integer, Double, Bool, Int)
allTypes = liftM (,,,,,,) getRandom
                     `ap` getRandom
                     `ap` getRandom
                     `ap` getRandom
                     `ap` getRandom
                     `ap` getRandom
                     `ap` getRandom