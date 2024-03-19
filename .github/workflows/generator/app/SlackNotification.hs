module SlackNotification (failureSteps, successSteps) where

import qualified CustomAction.PostCINotification as Notification
import Data.Function ((&))
import qualified Workflow.GitHub.Actions as GHA
import qualified Workflow.GitHub.Actions.Predefined.AWS.ConfigureCredentials as ConfigureAWSCredentials

configureStep :: GHA.Step
configureStep =
  ConfigureAWSCredentials.step
    & ConfigureAWSCredentials.awsRegion "ap-northeast-1"
    & ConfigureAWSCredentials.roleToAssume "arn:aws:iam::208140986057:role/GHALambdaInvoker-Bedrock"

notifyFailureStep :: String -> GHA.Step
notifyFailureStep jobName =
  GHA.namedAs "Notify as Failure" $
    Notification.step $
      Notification.Params
        { Notification.status = Notification.FailureStatus jobName,
          Notification.beginTime = GHA.mkNeedsOutputExpression "preconditions" "begintime",
          Notification.reportName = "Integrity Check",
          Notification.mode = Notification.BranchMode
        }

notifySuccessStep :: GHA.Step
notifySuccessStep =
  GHA.namedAs "Notify as Success" $
    Notification.step $
      Notification.Params
        { Notification.status = Notification.SuccessStatus,
          Notification.beginTime = GHA.mkNeedsOutputExpression "preconditions" "begintime",
          Notification.reportName = "Integrity Check",
          Notification.mode = Notification.BranchMode
        }

failureSteps :: String -> [GHA.Step]
failureSteps jobName = [configureStep, notifyFailureStep jobName]

successSteps :: [GHA.Step]
successSteps = [configureStep, notifySuccessStep]
