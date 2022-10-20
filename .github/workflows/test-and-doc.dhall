let GithubActions =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let Checkout =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions/checkout.dhall

let InstallRust =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions-rs/toolchain.dhall

let RunCargo =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions-rs/cargo.dhall

let GoogleAuth =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/google-github-actions/auth.dhall

let SetupAWSCredentials =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/aws-actions/configure-aws-credentials.dhall

let List/map = https://prelude.dhall-lang.org/List/map

let List/concat = https://prelude.dhall-lang.org/List/concat

let Text/concatSep = https://prelude.dhall-lang.org/Text/concatSep

let DocumentDeployment = ../actions/deployment-doc-peridot/schema.dhall

let SlackNotifierAction =
      https://raw.githubusercontent.com/Pctg-x8/ci-notifications-post-invoker/master/schema.dhall

let Features = ./features.dhall

let depends =
      λ(deps : List Text) →
      λ(job : GithubActions.Job.Type) →
        job ⫽ { needs = Some deps }

let withConditionStep =
      λ(cond : Text) →
      λ(step : GithubActions.Step.Type) →
        step ⫽ { if = Some cond }

let runStepOnFailure = withConditionStep "failure()"

let configureSlackNotification =
      SetupAWSCredentials.step
        SetupAWSCredentials.Params::{
        , awsRegion = "ap-northeast-1"
        , roleToAssume = Some
            "arn:aws:iam::208140986057:role/GHALambdaInvoker-Bedrock"
        }

let slackNotifyIfFailureStep =
      λ(stepName : Text) →
        SlackNotifierAction.step
          { status = SlackNotifierAction.Status.Failure stepName
          , begintime =
              GithubActions.mkExpression "needs.preconditions.outputs.begintime"
          , report_name = "Integrity Check"
          , mode = SlackNotifierAction.Mode.Branch
          }

let slackNotifySuccessStep =
      SlackNotifierAction.step
        { status = SlackNotifierAction.Status.Success
        , begintime =
            GithubActions.mkExpression "needs.preconditions.outputs.begintime"
        , report_name = "Integrity Check"
        , mode = SlackNotifierAction.Mode.Branch
        }

let preconditionRecordBeginTimeStep =
      GithubActions.Step::{
      , name = "Getting begintime"
      , id = Some "begintime"
      , run = Some "echo \"begintime=\$(date +%s)\" >> \$GITHUB_OUTPUT"
      }

let preconditions =
      GithubActions.Job::{
      , name = Some "Preconditions"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , outputs = Some
          ( toMap
              { begintime =
                  GithubActions.mkRefStepOutputExpression
                    "begintime"
                    "begintime"
              }
          )
      , steps = [ preconditionRecordBeginTimeStep ]
      }

let checkFormatStep =
      GithubActions.Job::{
      , name = Some "Check Format"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , steps =
        [ Checkout.stepv3 Checkout.Params::{=}
        , InstallRust.step InstallRust.Params::{ toolchain = Some "stable" }
        , RunCargo.step
            RunCargo.Params::{ command = "fmt", args = Some "-- --check" }
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (   slackNotifyIfFailureStep "Check Format"
              ⫽ { name = "Notify as Failure" }
            )
        ]
      }

let testStep =
      GithubActions.Job::{
      , name = Some "Run Tests (Platform Independent)"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , steps =
        [ Checkout.stepv3 Checkout.Params::{=}
        , InstallRust.step InstallRust.Params::{ toolchain = Some "stable" }
        , RunCargo.step
            RunCargo.Params::{
            , command = "test"
            , args = Some
                "--features ${Text/concatSep "," Features.PlatformIndependent}"
            }
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (   slackNotifyIfFailureStep "Run Tests"
              ⫽ { name = "Notify as Failure" }
            )
        ]
      }

let testStepWin32 =
      GithubActions.Job::{
      , name = Some "Run Tests (Win32 Specific)"
      , runs-on = GithubActions.RunnerPlatform.windows-latest
      , steps =
        [ Checkout.stepv3 Checkout.Params::{=}
        , InstallRust.step InstallRust.Params::{ toolchain = Some "stable" }
        , RunCargo.step
            RunCargo.Params::{
            , command = "check"
            , args = Some
                "--features ${Text/concatSep "," Features.Win32Specific}"
            }
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (   slackNotifyIfFailureStep "Run Tests (Win32 Specific)"
              ⫽ { name = "Notify as Failure" }
            )
        ]
      }

let testStepUnix =
      GithubActions.Job::{
      , name = Some "Run Tests (Unix Specific)"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , steps =
        [ Checkout.stepv3 Checkout.Params::{=}
        , InstallRust.step InstallRust.Params::{ toolchain = Some "stable" }
        , RunCargo.step
            RunCargo.Params::{
            , command = "check"
            , args = Some
                "--features ${Text/concatSep "," Features.UnixSpecific}"
            }
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (   slackNotifyIfFailureStep "Run Tests (Unix Specific)"
              ⫽ { name = "Notify as Failure" }
            )
        ]
      }

let testStepMac =
      GithubActions.Job::{
      , name = Some "Run Tests (Mac Specific)"
      , runs-on = GithubActions.RunnerPlatform.macos-latest
      , steps =
        [ Checkout.stepv3 Checkout.Params::{=}
        , InstallRust.step InstallRust.Params::{ toolchain = Some "stable" }
        , RunCargo.step
            RunCargo.Params::{
            , command = "check"
            , args = Some
                "--features ${Text/concatSep "," Features.MacSpecific}"
            }
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (   slackNotifyIfFailureStep "Run Tests (Mac Specific)"
              ⫽ { name = "Notify as Failure" }
            )
        ]
      }

let documentDeploymentStep =
      GithubActions.Job::{
      , name = Some "Deploy Latest Document"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , permissions = Some (toMap { id-token = "write" })
      , steps =
        [ Checkout.stepv3 Checkout.Params::{=}
        , InstallRust.step InstallRust.Params::{ toolchain = Some "nightly" }
        , RunCargo.step
            RunCargo.Params::{
            , command = "rustdoc"
            , args = Some
                "--features ${Text/concatSep
                                ","
                                ( List/concat
                                    Text
                                    [ Features.PlatformIndependent
                                    , Features.UnixSpecific
                                    , Features.MacSpecific
                                    ]
                                )} -- --cfg docsrs"
            , toolchain = Some "nightly"
            }
        , GoogleAuth.step
            GoogleAuth.Params::{
            , workloadIdentityProvider = Some
                "projects/146152181631/locations/global/workloadIdentityPools/github-actions-oidc-federation/providers/github-actions"
            , serviceAccount = Some
                "github-actions-autodeployer@docs-541f3.iam.gserviceaccount.com"
            , audience = Some "https://github.com/Pctg-x8"
            }
        , DocumentDeployment.step
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (   slackNotifyIfFailureStep "Deploy Latest Document"
              ⫽ { name = "Notify as Failure" }
            )
        ]
      }

let reportSuccessJob =
      GithubActions.Job::{
      , name = Some "Report as Success"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , steps =
        [ Checkout.stepv3 Checkout.Params::{=}
        , configureSlackNotification
        , slackNotifySuccessStep
        ]
      }

in  GithubActions.Workflow::{
    , name = Some "Integrity Check"
    , on = GithubActions.On.Single GithubActions.UnparameterizedTrigger.push
    , permissions = Some (toMap { id-token = "write", contents = "read" })
    , jobs = toMap
        { preconditions
        , check-format = depends [ "preconditions" ] checkFormatStep
        , test = depends [ "preconditions" ] testStep
        , test-win32 = depends [ "preconditions", "test" ] testStepWin32
        , test-unix = depends [ "preconditions", "test" ] testStepUnix
        , test-mac = depends [ "preconditions", "test" ] testStepMac
        , document-deploy =
            depends
              [ "preconditions"
              , "test"
              , "check-format"
              , "test-win32"
              , "test-unix"
              , "test-mac"
              ]
              documentDeploymentStep
        , report-success =
            depends
              [ "preconditions"
              , "test"
              , "check-format"
              , "document-deploy"
              , "test-win32"
              , "test-unix"
              , "test-mac"
              ]
              reportSuccessJob
        }
    }
