let GithubActions =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let InstallRust =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions-rs/toolchain.dhall

let RunCargo =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions-rs/cargo.dhall

let GoogleAuth =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/google-github-actions/auth.dhall

let Optional/default = https://prelude.dhall-lang.org/Optional/default

let DocumentDeployment = ../actions/deployment-doc-peridot/schema.dhall

let Features = ./features.dhall

let helper = ./helper.dhall

let JobBuilder = ./jobBuilder.dhall

let SlackNotification = ./slackNotification.dhall

let faultableJob =
      λ(job : GithubActions.Job.Type) →
        let jobName = Optional/default Text "<unknown job>" job.name

        in    job
            ⫽ { steps =
                  helper.flattenSteps
                    [ job.steps
                    , helper.runStepsOnFailure
                        (SlackNotification.notifyFailureSteps jobName)
                    ]
              }

let useRust =
      λ(toolchain : Text) →
      λ(job : GithubActions.Job.Type) →
          job
        ⫽ { steps =
              helper.prependStep
                ( InstallRust.step
                    InstallRust.Params::{ toolchain = Some toolchain }
                )
                job.steps
          }

let cargo =
      λ(subcommand : Text) →
      λ(args : Text) →
        RunCargo.step
          RunCargo.Params::{ command = subcommand, args = Some args }

let cargoNight =
      λ(subcommand : Text) →
      λ(args : Text) →
        RunCargo.step
          RunCargo.Params::{
          , command = subcommand
          , args = Some args
          , toolchain = Some "nightly"
          }

let steps =
      { simpleTestRustWithFeatures =
          λ(features : List Text) →
            cargo "test" "--features ${helper.serializeFeatures features}"
      , simpleCheckRustWithFeatures =
          λ(features : List Text) →
            cargo "check" "--features ${helper.serializeFeatures features}"
      }

let preconditions =
      let recordBeginTime =
            GithubActions.Step::{
            , name = "Getting begintime"
            , id = Some "begintime"
            , run = Some "echo \"begintime=\$(date +%s)\" >> \$GITHUB_OUTPUT"
            }

      in  JobBuilder.buildJob
            [ JobBuilder.output
                "begintime"
                ( GithubActions.mkRefStepOutputExpression
                    "begintime"
                    "begintime"
                )
            , JobBuilder.name "Preconditions"
            ]
            [ recordBeginTime ]

let checkFormat =
      JobBuilder.buildJob
        [ faultableJob
        , JobBuilder.useRepositoryContent
        , useRust "stable"
        , JobBuilder.name "Check Format"
        ]
        [ GithubActions.Step::{
          , name = "check fmt"
          , run = Some "cargo fmt -- --check"
          }
        ]

let platformIndependentTest =
      JobBuilder.buildJob
        [ faultableJob
        , JobBuilder.useRepositoryContent
        , useRust "stable"
        , JobBuilder.name "Run Tests (Platform Independent)"
        ]
        [ steps.simpleTestRustWithFeatures Features.PlatformIndependent ]

let platformDependentTests =
      { win32 =
          JobBuilder.buildJob
            [ faultableJob
            , JobBuilder.useRepositoryContent
            , useRust "stable"
            , JobBuilder.runner GithubActions.RunnerPlatform.windows-latest
            , JobBuilder.name "Run Tests (Win32 Specific)"
            ]
            [ steps.simpleCheckRustWithFeatures Features.Win32Specific ]
      , unix =
          JobBuilder.buildJob
            [ faultableJob
            , JobBuilder.useRepositoryContent
            , useRust "stable"
            , JobBuilder.name "Run Tests (Unix Specific)"
            ]
            [ steps.simpleCheckRustWithFeatures Features.UnixSpecific ]
      , mac =
          JobBuilder.buildJob
            [ faultableJob
            , JobBuilder.useRepositoryContent
            , useRust "stable"
            , JobBuilder.runner GithubActions.RunnerPlatform.macos-latest
            , JobBuilder.name "Run Tests (Mac Specific)"
            ]
            [ steps.simpleCheckRustWithFeatures Features.MacSpecific ]
      }

let documentDeploymentStep =
      let docFeatures = helper.serializeFeatures Features.ForDocumentation

      let buildDocument =
            cargoNight "rustdoc" "--features ${docFeatures} -- --cfg docsrs"

      let deploymentSteps =
            [ GoogleAuth.step
                GoogleAuth.Params::{
                , workloadIdentityProvider = Some
                    "projects/146152181631/locations/global/workloadIdentityPools/github-actions-oidc-federation/providers/github-actions"
                , serviceAccount = Some
                    "github-actions-autodeployer@docs-541f3.iam.gserviceaccount.com"
                , audience = Some "https://github.com/Pctg-x8"
                }
            , DocumentDeployment.step
            ]

      in  JobBuilder.buildJob
            [ faultableJob
            , JobBuilder.useRepositoryContent
            , useRust "nightly"
            , JobBuilder.requestIDTokenWritePermission
            , JobBuilder.name "Deploy Latest Document"
            ]
            (helper.prependStep buildDocument deploymentSteps)

let reportSuccessJob =
      JobBuilder.buildJob
        [ JobBuilder.useRepositoryContent, JobBuilder.name "Report as Success" ]
        SlackNotification.notifySuccessSteps

let checkJobs =
        JobBuilder.requireJobBefore
          (toMap { test = platformIndependentTest })
          ( toMap
              { test-win32 = platformDependentTests.win32
              , test-unix = platformDependentTests.unix
              , test-mac = platformDependentTests.mac
              }
          )
      # toMap { check-format = checkFormat }

let allJobs =
      JobBuilder.requireJobBefore
        (toMap { preconditions })
        ( JobBuilder.requireJobBefore
            checkJobs
            (toMap { document-deploy = documentDeploymentStep })
        )

in  GithubActions.Workflow::{
    , name = Some "Integrity Check"
    , on = GithubActions.On.Single GithubActions.UnparameterizedTrigger.push
    , permissions = Some (toMap { id-token = "write", contents = "read" })
    , jobs =
        JobBuilder.requireJobBefore
          allJobs
          (toMap { report-success = reportSuccessJob })
    }
