let GithubActions =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let SetupAWSCredentials =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/aws-actions/configure-aws-credentials.dhall

let SlackNotifierAction =
      https://raw.githubusercontent.com/Pctg-x8/ci-notifications-post-invoker/master/schema.dhall

let configureStep =
      SetupAWSCredentials.step
        SetupAWSCredentials.Params::{
        , awsRegion = "ap-northeast-1"
        , roleToAssume = Some
            "arn:aws:iam::208140986057:role/GHALambdaInvoker-Bedrock"
        }

let notifyFailureStep =
      λ(stepName : Text) →
          SlackNotifierAction.step
            { status = SlackNotifierAction.Status.Failure stepName
            , begintime =
                GithubActions.mkExpression
                  "needs.preconditions.outputs.begintime"
            , report_name = "Integrity Check"
            , mode = SlackNotifierAction.Mode.Branch
            }
        ⫽ { name = "Notify as Failure" }

let notifySuccessStep =
      SlackNotifierAction.step
        { status = SlackNotifierAction.Status.Success
        , begintime =
            GithubActions.mkExpression "needs.preconditions.outputs.begintime"
        , report_name = "Integrity Check"
        , mode = SlackNotifierAction.Mode.Branch
        }

let notifyFailureSteps =
      λ(stepName : Text) → [ configureStep, notifyFailureStep stepName ]

let notifySuccessSteps = [ configureStep, notifySuccessStep ]

in  { configureStep
    , notifyFailureStep
    , notifySuccessStep
    , notifyFailureSteps
    , notifySuccessSteps
    }
