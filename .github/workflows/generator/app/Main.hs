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
import qualified Workflow.GitHub.Actions.Predefined.Rust.Toolchain as RustToolchain

faultableJob :: GHA.Job -> GHA.Job
faultableJob job = GHA.jobModifySteps (<> steps) job
  where
    jobName = fromMaybe "<unknown job>" $ GHA.nameOf job
    steps = GHA.withCondition "failure()" <$> SlackNotification.failureSteps jobName

useRepositoryContent :: GHA.Job -> GHA.Job
useRepositoryContent = GHA.jobModifySteps (Checkout.step Nothing :)

useRust :: String -> GHA.Job -> GHA.Job
useRust toolchain = GHA.jobModifySteps ((RustToolchain.step & RustToolchain.useToolchain toolchain) :)

cargo :: String -> [String] -> GHA.Step
cargo subcommand args = GHA.runStep $ unwords ("cargo" : subcommand : args)

cargoNight :: String -> [String] -> GHA.Step
cargoNight subcommand args = GHA.runStep $ unwords ("cargo" : "+nightly" : subcommand : args)

simpleTestRustWithFeaturesStep :: [String] -> GHA.Step
simpleTestRustWithFeaturesStep features = cargo "test" ["--features", intercalate "," features]

simpleCheckRustWithFeaturesStep :: [String] -> GHA.Step
simpleCheckRustWithFeaturesStep features = cargo "check" ["--features", intercalate "," features]

preconditions :: GHA.Job
preconditions = GHA.jobForwardingStepOutput "begintime" "begintime" $ GHA.job [recordBeginTime]
  where
    recordBeginTime =
      GHA.namedAs "Getting begintime" $
        GHA.identifiedAs "begintime" $
          GHA.runStep "echo \"begintime=$(date +%s)\" >> $GITHUB_OUTPUT"

checkFormat :: GHA.Job
checkFormat = faultableJob $ GHA.namedAs "Check Format" $ useRust "stable" $ useRepositoryContent $ GHA.job [GHA.namedAs "check fmt" $ GHA.runStep "cargo fmt -- --check"]

platformIndependentTest :: GHA.Job
platformIndependentTest = faultableJob $ GHA.namedAs "Run Tests (Platform Independent)" $ useRust "stable" $ useRepositoryContent $ GHA.job [simpleTestRustWithFeaturesStep Features.platformIndependent]

win32DependentTest :: GHA.Job
win32DependentTest = faultableJob $ GHA.namedAs "Run Tests (Win32 Specific)" $ useRust "stable" $ useRepositoryContent $ GHA.jobRunsOn ["windows-latest"] $ GHA.job [simpleCheckRustWithFeaturesStep Features.win32Specific]

unixDependentTest :: GHA.Job
unixDependentTest = faultableJob $ GHA.namedAs "Run Tests (Unix Specific)" $ useRust "stable" $ useRepositoryContent $ GHA.job [simpleCheckRustWithFeaturesStep Features.unixSpecific]

macDependentTest :: GHA.Job
macDependentTest = faultableJob $ GHA.namedAs "Run Tests (Mac Specific)" $ useRust "stable" $ useRepositoryContent $ GHA.jobRunsOn ["macos-latest"] $ GHA.job [simpleCheckRustWithFeaturesStep Features.macSpecific]

documentDeploymentJob :: GHA.Job
documentDeploymentJob = faultableJob $ GHA.namedAs "Deploy Latest Document" $ useRust "nightly" $ useRepositoryContent $ GHA.grantWritable GHA.IDTokenPermission $ GHA.job (buildDocument : deploymentSteps)
  where
    buildDocument = cargoNight "rustdoc" ["--features", intercalate "," Features.forDocumentation, "--", "--cfg", "docsrs"]
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
