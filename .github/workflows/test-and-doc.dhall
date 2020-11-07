let GithubActions = https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall
let ProvidedSteps = https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps.dhall
let List/map = https://prelude.dhall-lang.org/List/map
let List/concat = https://prelude.dhall-lang.org/List/concat
let DocumentDeployment = ../actions/deployment-doc-peridot/schema.dhall
let SlackNotifierAction = https://raw.githubusercontent.com/Pctg-x8/ci-notifications-post-invoker/master/schema.dhall

let eSecretGithubToken = GithubActions.mkExpression "secrets.GITHUB_TOKEN"
let eSecretAWSAccessKey = GithubActions.mkExpression "secrets.AWS_ACCESS_KEY_ID"
let eSecretAWSAccessSecret = GithubActions.mkExpression "secrets.AWS_ACCESS_SECRET"
let eSecretFirebaseToken = GithubActions.mkExpression "secrets.DOC_HOST_FIREBASE_TOKEN"

let depends = \(deps: List Text) -> \(job: GithubActions.Job.Type) -> job // { needs = Some deps }
let withConditionStep = \(cond: Text) -> \(step: GithubActions.Step.Type) -> step // { `if` = Some cond }
let runStepOnFailure = withConditionStep "failure()"

let awsAccessEnvParams : SlackNotifierAction.ExecEnv =
    { AWS_ACCESS_KEY_ID = eSecretAWSAccessKey
    , AWS_SECRET_ACCESS_KEY = eSecretAWSAccessSecret
    , AWS_DEFAULT_REGION = "ap-northeast-1"
    }

let checkoutStep = ProvidedSteps.checkoutStep ProvidedSteps.CheckoutParams::{=}

let slackNotifyIfFailureStep = \(stepName: Text) -> SlackNotifierAction.step {
    , status = SlackNotifierAction.Status.Failure stepName
    , begintime = GithubActions.mkExpression "needs.preconditions.outputs.begintime"
    , report_name = "Integrity Check"
    , mode = SlackNotifierAction.Mode.Branch
    } awsAccessEnvParams
let slackNotifySuccessStep = SlackNotifierAction.step {
    , status = SlackNotifierAction.Status.Success
    , begintime = GithubActions.mkExpression "needs.preconditions.outputs.begintime"
    , report_name = "Integrity Check"
    , mode = SlackNotifierAction.Mode.Branch
    } awsAccessEnvParams

let preconditionRecordBeginTimeStep = GithubActions.Step::{
    , name = "Getting begintime"
    , id = Some "begintime"
    , run = Some "echo \"::set-output name=begintime::$(date +%s)\""
    }
let preconditionBeginTimestampOutputDef = toMap {
    , begintime = GithubActions.mkExpression "steps.begintime.outputs.begintime"
    }
let preconditions = GithubActions.Job::{
    , name = Some "Preconditions"
    , `runs-on` = GithubActions.RunnerPlatform.ubuntu-latest
    , outputs = Some (preconditionBeginTimestampOutputDef # toMap {
        , has_code_changes = GithubActions.mkExpression "steps.fileck.outputs.has_code_changes"
        , has_workflow_changes = GithubActions.mkExpression "steps.fileck.outputs.has_workflow_changes"
        })
    , steps = [
        , preconditionRecordBeginTimeStep
        ]
    }

let installDhallScript =
    let releaseAssetSelector = "nodes { name, downloadUrl }, pageInfo { hasNextPage, endCursor }"
    let releaseSelector = "nodes { releaseAssets(first: 10, after: $cursor) { ${ releaseAssetSelector } } }"
    let repositorySelector = "releases(first: 1, orderBy: { direction: DESC, field: CREATED_AT }) { ${ releaseSelector } }"
    let query = "query($cursor: String) { repository(owner: \\\"dhall-lang\\\", name: \\\"dhall-haskell\\\") { ${ repositorySelector } } }"
    in ''
    QUERY_STRING='${query}'
    QUERY_CURSOR='null'
    TARGET_FILE=""
    while :; do
      POSTDATA="{ \"query\": \"$QUERY_STRING\", \"variables\": { \"cursor\": $QUERY_CURSOR } }"
      API_RESPONSE=$(curl -s -H "Authorization: Bearer ${eSecretGithubToken}" -X POST -d "$POSTDATA" https://api.github.com/graphql)
      TARGET_FILE=$(echo $API_RESPONSE | jq -r '.data.repository.releases.nodes[0].releaseAssets.nodes[] | select(.name | startswith("dhall-yaml") and contains("-linux")).downloadUrl')
      if [[ $TARGET_FILE != "" ]]; then break; fi
      HAS_NEXT_PAGE=$(echo $API_RESPONSE | jq ".data.repository.releases.nodes[0].releaseAssets.pageInfo.hasNextPage")
      if [[ "$HAS_NEXT_PAGE" == "true" ]]; then
        QUERY_CURSOR=$(echo $API_RESPONSE | jq ".data.repository.releases.nodes[0].releaseAssets.pageINfo.endCursor")
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
let checkWorkflowSync = GithubActions.Job::{
    , name = Some "Check Workflow Files are Synchronized"
    , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
    , steps = [
        , checkoutStep
        , GithubActions.Step::{ name = "Setup Dhall", run = Some installDhallScript }
        , GithubActions.Step::{ name = "test-sync", run = Some "make -C ./.github/workflows test-sync" }
        , runStepOnFailure (slackNotifyIfFailureStep  "check-sync-workflow" // { name = "Notify as Failure" })
        ]
    }
let testStep = GithubActions.Job::{
    , name = Some "Test build"
    , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
    , steps = [
        , checkoutStep
        , GithubActions.Step::{ name = "cargo test", run = Some "cargho test --features Presentation,VK_EXT_debug_report" }
        , runStepOnFailure (slackNotifyIfFailureStep "test" // { name = "Notify as Failure" })
        ]
    }
let documentDeploymentStep = GithubActions.Job::{
    , name = Some "Deploy Latest Document"
    , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
    , steps = [
        , checkoutStep
        , GithubActions.Step::{
            , name = "build document"
            , run = Some "cargo doc --no-deps --features=Implements,Presentation,VK_EXT_debug_report"
            }
        , DocumentDeployment.step
            { FirebaseToken = eSecretFirebaseToken
            }
        , runStepOnFailure (slackNotifyIfFailureStep "document-deploy" // { name = "Notify as Failure" })
        ]
    }

let reportSuccessJob = GithubActions.Job::{
    , name = Some "Report as Success"
    , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
    , steps = [
        , checkoutStep
        , slackNotifySuccessStep
        ]
    }

in GithubActions.Workflow::{
    , name = Some "Integrity Check"
    , on = GithubActions.On.Single GithubActions.UnparameterizedTrigger.push
    , jobs = toMap {
        , preconditions = preconditions
        , check-workflow-sync = depends ["preconditions"] checkWorkflowSync
        , test = depends ["check-workflow-sync", "preconditions"] testStep
        , document-deploy = depends ["check-workflow-sync", "preconditions"] documentDeploymentStep
        , report-success = depends ["preconditions", "check-workflow-sync", "test", "document-deploy"] reportSuccessJob
        }
    }
