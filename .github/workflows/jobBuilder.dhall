let GithubActions =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let Checkout =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions/checkout.dhall

let Map = https://prelude.dhall-lang.org/Map/Type

let Map/keys = https://prelude.dhall-lang.org/Map/keys

let Map/mapValue = https://prelude.dhall-lang.org/Map/map

let Optional/default = https://prelude.dhall-lang.org/Optional/default

let Optional/fold = https://prelude.dhall-lang.org/Optional/fold

let List/map = https://prelude.dhall-lang.org/List/map

let List/concat = https://prelude.dhall-lang.org/List/concat

let helper = ./helper.dhall

let DefaultRunnerPlatform = GithubActions.RunnerPlatform.ubuntu-latest

let JobMap = Map Text GithubActions.Job.Type

let JobModifier = GithubActions.Job.Type → GithubActions.Job.Type

let stdJob =
      λ(steps : List GithubActions.Step.Type) →
        GithubActions.Job::{ runs-on = DefaultRunnerPlatform, steps }

let singleStepJob = λ(step : GithubActions.Step.Type) → stdJob [ step ]

let name =
      λ(name : Text) →
      λ(job : GithubActions.Job.Type) →
        job ⫽ { name = Some name }

let runner =
      λ(platform : GithubActions.RunnerPlatform) →
      λ(job : GithubActions.Job.Type) →
        job ⫽ { runs-on = platform }

let output =
      λ(name : Text) →
      λ(value : Text) →
      λ(job : GithubActions.Job.Type) →
          job
        ⫽ { outputs = Some
              (   Optional/default
                    (Map Text Text)
                    ([] : Map Text Text)
                    job.outputs
                # [ { mapKey = name, mapValue = value } ]
              )
          }

let depends =
      λ(dep : Text) →
      λ(job : GithubActions.Job.Type) →
          job
        ⫽ { needs = Some
              ( Optional/fold
                  (List Text)
                  job.needs
                  (List Text)
                  (λ(current : List Text) → current # [ dep ])
                  [ dep ]
              )
          }

let useRepositoryContent =
      λ(job : GithubActions.Job.Type) →
          job
        ⫽ { steps =
              helper.prependStep
                (Checkout.stepv3 Checkout.Params::{=})
                job.steps
          }

let requestIDTokenWritePermission =
      λ(job : GithubActions.Job.Type) →
          job
        ⫽ { permissions = Some
              ( Optional/fold
                  (Map Text Text)
                  job.permissions
                  (Map Text Text)
                  ( λ(perms : Map Text Text) →
                      perms # toMap { id-token = "write" }
                  )
                  (toMap { id-token = "write" })
              )
          }

let applyJobModifiers =
      λ(modifiers : List JobModifier) →
      λ(target : GithubActions.Job.Type) →
        List/fold
          JobModifier
          modifiers
          GithubActions.Job.Type
          (λ(mod : JobModifier) → λ(x : GithubActions.Job.Type) → mod x)
          target

let buildJob =
      λ(modifiers : List JobModifier) →
      λ(steps : List GithubActions.Step.Type) →
        applyJobModifiers modifiers (stdJob steps)

let requireJobBefore =
      λ(prejobs : JobMap) →
      λ(afterJobs : JobMap) →
          prejobs
        # Map/mapValue
            Text
            GithubActions.Job.Type
            GithubActions.Job.Type
            ( applyJobModifiers
                ( List/map
                    Text
                    JobModifier
                    depends
                    (Map/keys Text GithubActions.Job.Type prejobs)
                )
            )
            afterJobs

in  { JobModifier
    , JobMap
    , name
    , runner
    , output
    , depends
    , applyJobModifiers
    , buildJob
    , useRepositoryContent
    , requestIDTokenWritePermission
    , requireJobBefore
    }
