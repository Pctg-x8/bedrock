let GithubActions =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let List/concat = https://prelude.dhall-lang.org/List/concat

let List/map = https://prelude.dhall-lang.org/List/map

let Text/concatSep = https://prelude.dhall-lang.org/Text/concatSep

let flattenSteps = List/concat GithubActions.Step.Type

let serializeFeatures = Text/concatSep ","

let withConditionStep =
      λ(cond : Text) →
      λ(step : GithubActions.Step.Type) →
        step ⫽ { `if` = Some cond }

let runStepOnFailure = withConditionStep "failure()"

let runStepsOnFailure =
      λ(steps : List GithubActions.Step.Type) →
        List/map
          GithubActions.Step.Type
          GithubActions.Step.Type
          runStepOnFailure
          steps

in  { flattenSteps
    , serializeFeatures
    , withConditionStep
    , runStepOnFailure
    , runStepsOnFailure
    }
