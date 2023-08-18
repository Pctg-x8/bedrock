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

let simpleTestRustWithFeatures =
      λ(features : List Text) →
        RunCargo.step
          RunCargo.Params::{
          , command = "test"
          , args = Some "--features ${helper.serializeFeatures features}"
          }

let simpleCheckRustWithFeatures =
      λ(features : List Text) →
        RunCargo.step
          RunCargo.Params::{
          , command = "check"
          , args = Some "--features ${helper.serializeFeatures features}"
          }

let preconditionRecordBeginTimeStep =
      GithubActions.Step::{
      , name = "Getting begintime"
      , id = Some "begintime"
      , run = Some "echo \"begintime=\$(date +%s)\" >> \$GITHUB_OUTPUT"
      }

let preconditions =
      JobBuilder.buildJob
        [ JobBuilder.jobOutput
            "begintime"
            (GithubActions.mkRefStepOutputExpression "begintime" "begintime")
        , JobBuilder.jobName "Preconditions"
        ]
        [ preconditionRecordBeginTimeStep ]

let checkFormat =
      JobBuilder.buildJob
        [ faultableJob
        , JobBuilder.useRepositoryContent
        , useRust "stable"
        , JobBuilder.jobName "Check Format"
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
        , JobBuilder.jobName "Run Tests (Platform Independent)"
        ]
        [ simpleTestRustWithFeatures Features.PlatformIndependent ]

let platformDependentTests =
      { win32 =
          JobBuilder.buildJob
            [ faultableJob
            , JobBuilder.useRepositoryContent
            , useRust "stable"
            , JobBuilder.jobRunner GithubActions.RunnerPlatform.windows-latest
            , JobBuilder.jobName "Run Tests (Win32 Specific)"
            ]
            [ simpleCheckRustWithFeatures Features.Win32Specific ]
      , unix =
          JobBuilder.buildJob
            [ faultableJob
            , JobBuilder.useRepositoryContent
            , useRust "stable"
            , JobBuilder.jobName "Run Tests (Unix Specific)"
            ]
            [ simpleCheckRustWithFeatures Features.UnixSpecific ]
      , mac =
          JobBuilder.buildJob
            [ faultableJob
            , JobBuilder.useRepositoryContent
            , useRust "stable"
            , JobBuilder.jobRunner GithubActions.RunnerPlatform.macos-latest
            , JobBuilder.jobName "Run Tests (Mac Specific)"
            ]
            [ simpleCheckRustWithFeatures Features.MacSpecific ]
      }

let documentDeploymentStep =
      let docFeatures = helper.serializeFeatures Features.ForDocumentation

      let buildDocument =
            RunCargo.step
              RunCargo.Params::{
              , command = "rustdoc"
              , args = Some "--features ${docFeatures} -- --cfg docsrs"
              , toolchain = Some "nightly"
              }

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
            , JobBuilder.jobName "Deploy Latest Document"
            ]
            (helper.prependStep buildDocument deploymentSteps)

let reportSuccessJob =
      JobBuilder.buildJob
        [ JobBuilder.useRepositoryContent
        , JobBuilder.jobName "Report as Success"
        ]
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
