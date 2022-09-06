let GithubActions =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let Checkout =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions/checkout.dhall

let InstallRust =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions-rs/toolchain.dhall

let RunCargo =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions-rs/cargo.dhall

let List/map = https://prelude.dhall-lang.org/List/map

let List/concat = https://prelude.dhall-lang.org/List/concat

let DocumentDeployment = ../actions/deployment-doc-peridot/schema.dhall

let SlackNotifierAction =
      https://raw.githubusercontent.com/Pctg-x8/ci-notifications-post-invoker/master/schema.dhall

let eSecretGithubToken = GithubActions.mkExpression "secrets.GITHUB_TOKEN"

let eSecretFirebaseToken =
      GithubActions.mkExpression "secrets.DOC_HOST_FIREBASE_TOKEN"

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

let checkoutStep = Checkout.step Checkout.Params::{=}

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

let preconditionBeginTimestampOutputDef =
      toMap
        { begintime =
            GithubActions.mkExpression "steps.begintime.outputs.begintime"
        }

let preconditions =
      GithubActions.Job::{
      , name = Some "Preconditions"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , outputs = Some
          (   preconditionBeginTimestampOutputDef
            # toMap
                { has_code_changes =
                    GithubActions.mkExpression
                      "steps.fileck.outputs.has_code_changes"
                , has_workflow_changes =
                    GithubActions.mkExpression
                      "steps.fileck.outputs.has_workflow_changes"
                }
          )
      , steps = [ preconditionRecordBeginTimeStep ]
      }

let installDhallScript =
      let releaseAssetSelector =
            "nodes { name, downloadUrl }, pageInfo { hasNextPage, endCursor }"

      let releaseSelector =
            "nodes { releaseAssets(first: 10, after: \$cursor) { ${releaseAssetSelector} } }"

      let repositorySelector =
            "releases(first: 1, orderBy: { direction: DESC, field: CREATED_AT }) { ${releaseSelector} }"

      let query =
            "query(\$cursor: String) { repository(owner: \\\"dhall-lang\\\", name: \\\"dhall-haskell\\\") { ${repositorySelector} } }"

      in  ''
          QUERY_STRING='${query}'
          QUERY_CURSOR='null'
          TARGET_FILE=""
          while :; do
            POSTDATA="{ \"query\": \"$QUERY_STRING\", \"variables\": { \"cursor\": $QUERY_CURSOR } }"
            API_RESPONSE=$(curl -s -H "Authorization: Bearer ${eSecretGithubToken}" -X POST -d "$POSTDATA" https://api.github.com/graphql)
            TARGET_FILE=$(echo $API_RESPONSE | jq -r '.data.repository.releases.nodes[0].releaseAssets.nodes[] | select(.name | startswith("dhall-yaml") and contains("-Linux")).downloadUrl')
            if [[ $TARGET_FILE != "" ]]; then break; fi
            HAS_NEXT_PAGE=$(echo $API_RESPONSE | jq ".data.repository.releases.nodes[0].releaseAssets.pageInfo.hasNextPage")
            if [[ "$HAS_NEXT_PAGE" == "true" ]]; then
              QUERY_CURSOR=$(echo $API_RESPONSE | jq ".data.repository.releases.nodes[0].releaseAssets.pageInfo.endCursor")
            else
              echo "Latest dhall release does not contains dhall-yaml for linux platform!"
              exit 1
            fi
          done < <(cat)
          echo "$TARGET_FILE"
          mkdir $HOME/dhall
          curl -L $TARGET_FILE | tar x --bzip2 -C $HOME/dhall
          echo "$HOME/dhall/bin" >> $GITHUB_PATH
          sudo apt-get update
          sudo apt-get install -y colordiff
          ''

let checkWorkflowSync =
      GithubActions.Job::{
      , name = Some "Check Workflow Files are Synchronized"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , steps =
        [ checkoutStep
        , GithubActions.Step::{
          , name = "Setup Dhall"
          , run = Some installDhallScript
          }
        , GithubActions.Step::{
          , name = "test-sync"
          , run = Some "make -C ./.github/workflows test-sync"
          }
        , runStepOnFailure configureSlackNotification
        , runStepOnFailure
            (     slackNotifyIfFailureStep "check-sync-workflow"
              //  { name = "Notify as Failure" }
            )
        ]
      }

let checkFormatStep =
      GithubActions.Job::{
      , name = Some "Check Format"
      , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
      , steps =
        [ checkoutStep
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
        [ checkoutStep
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
      , steps =
        [ checkoutStep
        , InstallRust.step InstallRust.Params::{ toolchain = Some "nightly" }
        , RunCargo.step
            RunCargo.Params::{
            , command = "rustdoc"
            , args = Some
                "--no-deps --features Implements,Multithreaded,Presentation,VK_EXT_debug_report -- --cfg docsrs"
            , toolchain = Some "nightly"
            }
        , DocumentDeployment.step { FirebaseToken = eSecretFirebaseToken }
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
        [ checkoutStep, configureSlackNotification, slackNotifySuccessStep ]
      }

in  GithubActions.Workflow::{
    , name = Some "Integrity Check"
    , on = GithubActions.On.Single GithubActions.UnparameterizedTrigger.push
    , jobs = toMap
        { preconditions
        , check-workflow-sync = depends [ "preconditions" ] checkWorkflowSync
        , check-format =
            depends [ "check-workflow-sync", "preconditions" ] checkFormatStep
        , test = depends [ "check-workflow-sync", "preconditions" ] testStep
        , document-deploy =
            depends
              [ "check-workflow-sync", "preconditions", "test", "check-format" ]
              documentDeploymentStep
        , report-success =
            depends
              [ "preconditions"
              , "check-workflow-sync"
              , "test"
              , "check-format"
              , "document-deploy"
              ]
              reportSuccessJob
        }
    }
