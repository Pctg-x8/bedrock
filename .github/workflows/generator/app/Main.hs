module Main (main) where

import qualified Data.ByteString.Lazy.Char8 as LBS8
import Data.Function ((&))
import Data.List (intercalate)
import Data.Map (Map)
import qualified Data.Map as M
import Data.Maybe (fromMaybe)
import qualified DocumentDeployment
import qualified Features
import qualified SlackNotification
import System.Environment (getArgs)
import System.FilePath ((</>))
import qualified Workflow.GitHub.Actions as GHA
import Workflow.GitHub.Actions.JobGroupComposer ((~=>))
import qualified Workflow.GitHub.Actions.Predefined.Checkout as Checkout
import qualified Workflow.GitHub.Actions.Predefined.Google.Auth as GoogleAuth

faultableJob :: GHA.Job -> GHA.Job
faultableJob job = GHA.jobModifySteps (<> steps) job
  where
    jobName = fromMaybe "<unknown job>" $ GHA.nameOf job
    steps = GHA.withCondition "failure()" <$> SlackNotification.failureSteps jobName

data Platform = Win32 | Unix | Mac

-- TODO: そのうちちゃんとlatest参照して引っ張ってくるとかしたい
-- TODO: これはcheckoutよりあとにやる必要がある 順序依存あるのいやだな......
downloadCargoTranslator :: Platform -> GHA.Step
downloadCargoTranslator Unix = GHA.runStep "curl -o ./cargo-json-gha-translator -L https://github.com/Pctg-x8/cargo-json-gha-translator/releases/download/v0.1.4/cargo-json-gha-translator-linux && chmod +x ./cargo-json-gha-translator"
downloadCargoTranslator Mac = GHA.runStep "curl -o ./cargo-json-gha-translator -L https://github.com/Pctg-x8/cargo-json-gha-translator/releases/download/v0.1.4/cargo-json-gha-translator-mac && chmod +x ./cargo-json-gha-translator"
downloadCargoTranslator Win32 = GHA.runStep "curl -o ./cargo-json-gha-translator.exe -L https://github.com/Pctg-x8/cargo-json-gha-translator/releases/download/v0.1.4/cargo-json-gha-translator-windows.exe"

useRepositoryContent :: GHA.Job -> GHA.Job
useRepositoryContent = GHA.jobModifySteps (Checkout.step Nothing :)

useRust :: String -> Platform -> GHA.Job -> GHA.Job
useRust toolchain pf = GHA.jobModifySteps \x -> GHA.runStep ("rustup set profile minimum && rustup install " <> toolchain <> " && rustup override set " <> toolchain) : downloadCargoTranslator pf : x

data Cargo = Cargo {cargoSubcommand :: String, cargoFeatures :: [String], cargoToolchainOverriding :: Maybe String, cargoMessageFormat :: Maybe String, cargoSubcommandInternalArgs :: [String]}

cargoRenderCommandline :: Cargo -> String
cargoRenderCommandline Cargo {..} = unwords $ mconcat [["cargo"], toolchainOverriding, [cargoSubcommand], features, msgformat, subcommandExtraArgs]
  where
    toolchainOverriding = maybe [] (\t -> ["+" <> t]) cargoToolchainOverriding
    features = if null cargoFeatures then [] else ["--features", intercalate "," cargoFeatures]
    msgformat = maybe [] (\t -> ["--message-format=" <> t]) cargoMessageFormat
    subcommandExtraArgs = if null cargoSubcommandInternalArgs then [] else "--" : cargoSubcommandInternalArgs

cargoRenderAnnotatedCommandline :: Cargo -> String
cargoRenderAnnotatedCommandline c = cargoRenderCommandline (cargoOutputJson c) <> " | ./cargo-json-gha-translator"

cargo :: String -> Cargo
cargo subcommand = Cargo {cargoSubcommand = subcommand, cargoFeatures = [], cargoToolchainOverriding = Nothing, cargoMessageFormat = Nothing, cargoSubcommandInternalArgs = []}

cargoOnNightly :: Cargo -> Cargo
cargoOnNightly c = c {cargoToolchainOverriding = Just "nightly"}

cargoWithFeatures :: [String] -> Cargo -> Cargo
cargoWithFeatures features c = c {cargoFeatures = features}

cargoSubcommandExtraArgs :: [String] -> Cargo -> Cargo
cargoSubcommandExtraArgs args c = c {cargoSubcommandInternalArgs = args}

cargoOutputJson :: Cargo -> Cargo
cargoOutputJson c = c {cargoMessageFormat = Just "json"}

preconditions :: GHA.Job
preconditions = GHA.jobForwardingStepOutput "begintime" "begintime" $ GHA.job [recordBeginTime]
  where
    recordBeginTime =
      GHA.namedAs "Getting begintime" $
        GHA.identifiedAs "begintime" $
          GHA.runStep "echo \"begintime=$(date +%s)\" >> $GITHUB_OUTPUT"

checkFormat :: GHA.Job
checkFormat = faultableJob $ GHA.namedAs "Check Format" $ useRepositoryContent $ useRust "stable" Unix $ GHA.job [GHA.namedAs "check fmt" $ GHA.runStep "cargo fmt -- --check"]

platformIndependentTest :: GHA.Job
platformIndependentTest =
  faultableJob $
    GHA.namedAs "Run Tests (Platform Independent)" $
      useRepositoryContent $
        useRust "stable" Unix $
          GHA.job
            [ GHA.namedAs "test (baseline)" $ GHA.runStep $ "set -o pipefail; " <> cargoRenderAnnotatedCommandline (cargo "test"),
              GHA.namedAs "test (featured)" $ GHA.runStep $ "set -o pipefail; " <> cargoRenderAnnotatedCommandline (cargo "test" & cargoWithFeatures Features.platformIndependent)
            ]

win32DependentTest :: GHA.Job
win32DependentTest =
  faultableJob $
    GHA.namedAs "Run Tests (Win32 Specific)" $
      useRepositoryContent $
        useRust "stable" Win32 $
          GHA.jobRunsOn ["windows-latest"] $
            GHA.job [GHA.runStep $ cargoRenderAnnotatedCommandline (cargo "check" & cargoWithFeatures Features.win32Specific) <> " || $(throw)"]

unixDependentTest :: GHA.Job
unixDependentTest =
  faultableJob $
    GHA.namedAs "Run Tests (Unix Specific)" $
      useRepositoryContent $
        useRust "stable" Unix $
          GHA.job [GHA.runStep $ "set -o pipefail; " <> cargoRenderAnnotatedCommandline (cargo "check" & cargoWithFeatures Features.unixSpecific)]

macDependentTest :: GHA.Job
macDependentTest =
  faultableJob $
    GHA.namedAs "Run Tests (Mac Specific)" $
      useRepositoryContent $
        useRust "stable" Mac $
          GHA.jobRunsOn ["macos-latest"] $
            GHA.job [GHA.runStep $ "set -o pipefail; " <> cargoRenderAnnotatedCommandline (cargo "check" & cargoWithFeatures Features.macSpecific)]

documentDeploymentJob :: GHA.Job
documentDeploymentJob = faultableJob $ GHA.namedAs "Deploy Latest Document" $ useRepositoryContent $ useRust "nightly" Unix $ GHA.grantWritable GHA.IDTokenPermission $ GHA.job (buildDocument : deploymentSteps)
  where
    buildDocument = GHA.runStep $ cargoRenderCommandline (cargo "rustdoc" & cargoOnNightly & cargoWithFeatures Features.forDocumentation & cargoSubcommandExtraArgs ["--cfg", "docsrs"])
    deploymentSteps =
      [ GHA.stepSetWithParam "audience" "https://github.com/Pctg-x8" $ GoogleAuth.viaWorkloadIdentityStep "projects/146152181631/locations/global/workloadIdentityPools/github-actions-oidc-federation/providers/github-actions" "github-actions-autodeployer@docs-541f3.iam.gserviceaccount.com",
        DocumentDeployment.step
      ]

reportSuccessJob :: GHA.Job
reportSuccessJob = GHA.namedAs "Report as Success" $ useRepositoryContent $ GHA.job SlackNotification.successSteps

jobs :: Map String GHA.Job
jobs =
  let preconditions' = M.singleton "preconditions" preconditions
      checkFormat' = M.singleton "checkFormat" checkFormat
      platformIndependentTest' = M.singleton "platformIndependentTest" platformIndependentTest
      win32DependentTest' = M.singleton "test-win32" win32DependentTest
      unixDependentTest' = M.singleton "test-unix" unixDependentTest
      macDependentTest' = M.singleton "test-mac" macDependentTest
      documentDeploymentJob' = M.singleton "documentDeployment" documentDeploymentJob
      reportSuccessJob' = M.singleton "reportSuccessJob" reportSuccessJob
   in preconditions' ~=> [checkFormat', platformIndependentTest' ~=> [win32DependentTest', unixDependentTest', macDependentTest']] ~=> documentDeploymentJob' ~=> reportSuccessJob'

integrityTest :: GHA.Workflow
integrityTest =
  GHA.buildWorkflow
    [ GHA.namedAs "Integrity Check",
      GHA.grantWritable GHA.IDTokenPermission,
      GHA.grantReadable GHA.ContentsPermission,
      GHA.workflowReplaceJobs jobs
    ]
    $ GHA.onPush GHA.workflowPushTrigger

main :: IO ()
main = do
  basePath <- head <$> getArgs
  LBS8.writeFile (basePath </> "test-and-doc.yml") $ GHA.build integrityTest
