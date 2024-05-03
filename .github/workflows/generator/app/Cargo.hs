module Cargo (Cargo, renderCommandline, cargo, onNightly, withFeatures, subcommandExtraArgs, outputJson) where

import Data.List (intercalate)

data Cargo = Cargo {cargoSubcommand :: String, cargoFeatures :: [String], cargoToolchainOverriding :: Maybe String, cargoMessageFormat :: Maybe String, cargoSubcommandInternalArgs :: [String]}

renderCommandline :: Cargo -> String
renderCommandline Cargo {..} = unwords $ mconcat [["cargo"], toolchainOverriding, [cargoSubcommand], features, msgformat, subcommandExtraArgs']
  where
    toolchainOverriding = maybe [] (\t -> ["+" <> t]) cargoToolchainOverriding
    features = if null cargoFeatures then [] else ["--features", intercalate "," cargoFeatures]
    msgformat = maybe [] (\t -> ["--message-format=" <> t]) cargoMessageFormat
    subcommandExtraArgs' = if null cargoSubcommandInternalArgs then [] else "--" : cargoSubcommandInternalArgs

cargo :: String -> Cargo
cargo subcommand = Cargo {cargoSubcommand = subcommand, cargoFeatures = [], cargoToolchainOverriding = Nothing, cargoMessageFormat = Nothing, cargoSubcommandInternalArgs = []}

onNightly :: Cargo -> Cargo
onNightly c = c {cargoToolchainOverriding = Just "nightly"}

withFeatures :: [String] -> Cargo -> Cargo
withFeatures features c = c {cargoFeatures = features}

subcommandExtraArgs :: [String] -> Cargo -> Cargo
subcommandExtraArgs args c = c {cargoSubcommandInternalArgs = args}

outputJson :: Cargo -> Cargo
outputJson c = c {cargoMessageFormat = Just "json"}
