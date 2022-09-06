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

let List/map = https://prelude.dhall-lang.org/List/map

let List/concat = https://prelude.dhall-lang.org/List/concat

let DocumentDeployment = ../actions/deployment-doc-peridot/schema.dhall

let SlackNotifierAction =
      https://raw.githubusercontent.com/Pctg-x8/ci-notifications-post-invoker/master/schema.dhall

let depends =
      \(deps : List Text) ->
      \(job : GithubActions.Job.Type) ->
        job // { needs = Some deps }

let withConditionStep =
      \(cond : Text) ->
      \(step : GithubActions.Step.Type) ->
        step // { `if` = Some cond }

let runStepOnFailure = withConditionStep "failure()"

let configureSlackNotification =
      GithubActions.Step::{
      , name = "Configure for Slack Notification"
      , id = Some "cfgNotification"
      , run = Some
          ''
          # re-export configs for further step
          echo AWS_ROLE_ARN=$AWS_ROLE_ARN >> $GITHUB_ENV
          echo AWS_WEB_IDENTITY_TOKEN_FILE=$AWS_WEB_IDENTITY_TOKEN_FILE >> $GITHUB_ENV
          echo AWS_DEFAULT_REGION=$AWS_DEFAULT_REGION >> $GITHUB_ENV

          curl -H "Authorization: Bearer $ACTIONS_ID_TOKEN_REQUEST_TOKEN" "$ACTIONS_ID_TOKEN_REQUEST_URL&audience=https://github.com/Pctg-x8/bedrock" | jq -r ".value" > $AWS_WEB_IDENTITY_TOKEN_FILE
          ''
      , env = Some
          ( toMap
              { AWS_ROLE_ARN = "arn:aws:iam::208140986057:role/GHALambdaInvoker"
              , AWS_WEB_IDENTITY_TOKEN_FILE = "/tmp/awstoken"
              , AWS_DEFAULT_REGION = "ap-northeast-1"
              }
          )
      }

let slackNotifyIfFailureStep =
      \(stepName : Text) ->
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
      , run = Some "echo \"::set-output name=begintime::\$(date +%s)\""
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
        [ Checkout.step Checkout.Params::{=}
        , InstallRust.step InstallRust.Params::{ toolchain = Some "stable" }
        , RunCargo.step
            RunCargo.Params::{ command = "fmt", args = Some "-- --check" }
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (     slackNotifyIfFailureStep "Check Format"
              //  { name = "Notify as Failure" }
            )
        ]
      }

let testStep =
      GithubActions.Job::{
      , name = Some "Run Tests"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , steps =
        [ Checkout.step Checkout.Params::{=}
        , InstallRust.step InstallRust.Params::{ toolchain = Some "stable" }
        , RunCargo.step
            RunCargo.Params::{
            , command = "test"
            , args = Some "--features Presentation,VK_EXT_debug_report"
            }
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (     slackNotifyIfFailureStep "Run Tests"
              //  { name = "Notify as Failure" }
            )
        ]
      }

let documentDeploymentStep =
      GithubActions.Job::{
      , name = Some "Deploy Latest Document"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , permissions = Some (toMap { id-token = "write" })
      , steps =
        [ Checkout.step Checkout.Params::{=}
        , InstallRust.step InstallRust.Params::{ toolchain = Some "nightly" }
        , RunCargo.step
            RunCargo.Params::{
            , command = "rustdoc"
            , args = Some
                "--features Implements,Multithreaded,Presentation,VK_EXT_debug_report -- --cfg docsrs"
            , toolchain = Some "nightly"
            }
        , GoogleAuth.step
            GoogleAuth.Params::{
            , workloadIdentityProvider = Some
                "projects/459189735048/locations/global/workloadIdentityPools/github-actions-oidc-federation/providers/github-actions"
            , serviceAccount = Some
                "bedrock-docs-autodeployer@nobilling-329804.iam.gserviceaccount.com"
            , audience = Some "https://github.com/Pctg-x8"
            }
        , DocumentDeployment.step
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (     slackNotifyIfFailureStep "Deploy Latest Document"
              //  { name = "Notify as Failure" }
            )
        ]
      }

let reportSuccessJob =
      GithubActions.Job::{
      , name = Some "Report as Success"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , steps =
        [ Checkout.step Checkout.Params::{=}
        , configureSlackNotification
        , slackNotifySuccessStep
        ]
      }

in  GithubActions.Workflow::{
    , name = Some "Integrity Check"
    , on = GithubActions.On.Single GithubActions.UnparameterizedTrigger.push
    , jobs = toMap
        { preconditions
        , check-format = depends [ "preconditions" ] checkFormatStep
        , test = depends [ "preconditions" ] testStep
        , document-deploy =
            depends
              [ "preconditions", "test", "check-format" ]
              documentDeploymentStep
        , report-success =
            depends
              [ "preconditions", "test", "check-format", "document-deploy" ]
              reportSuccessJob
        }
    }
