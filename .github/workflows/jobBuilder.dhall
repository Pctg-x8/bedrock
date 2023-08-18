let GithubActions =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let Checkout =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/ProvidedSteps/actions/checkout.dhall

let Map = https://prelude.dhall-lang.org/Map/Type

let Map/keyValue = https://prelude.dhall-lang.org/Map/keyValue

let Map/keys = https://prelude.dhall-lang.org/Map/keys

let Map/mapValue = https://prelude.dhall-lang.org/Map/map

let Optional/default = https://prelude.dhall-lang.org/Optional/default

let List/map = https://prelude.dhall-lang.org/List/map

let DefaultRunnerPlatform = GithubActions.RunnerPlatform.ubuntu-latest

let JobMap = Map Text GithubActions.Job.Type

let JobOutputMap = Map Text Text

let JobOutputMap/entry = Map/keyValue Text

let JobModifier = GithubActions.Job.Type → GithubActions.Job.Type

let stdJob =
      λ(steps : List GithubActions.Step.Type) →
        GithubActions.Job::{ runs-on = DefaultRunnerPlatform, steps }

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
        let outputs =
              Optional/default JobOutputMap ([] : JobOutputMap) job.outputs

        in    job
            ⫽ { outputs = Some (outputs # [ JobOutputMap/entry name value ]) }

let depends =
      λ(dep : Text) →
      λ(job : GithubActions.Job.Type) →
        let needs = Optional/default (List Text) ([] : List Text) job.needs

        in  job ⫽ { needs = Some (needs # [ dep ]) }

let useRepositoryContent =
      λ(job : GithubActions.Job.Type) →
        job ⫽ { steps = [ Checkout.stepv3 Checkout.Params::{=} ] # job.steps }

let requestIDTokenWritePermission =
      λ(job : GithubActions.Job.Type) →
        let permissions =
              Optional/default
                (Map Text Text)
                ([] : Map Text Text)
                job.permissions

        in    job
            ⫽ { permissions = Some (permissions # toMap { id-token = "write" })
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
        let requiredJobNames = Map/keys Text GithubActions.Job.Type prejobs

        in    prejobs
            # Map/mapValue
                Text
                GithubActions.Job.Type
                GithubActions.Job.Type
                ( applyJobModifiers
                    (List/map Text JobModifier depends requiredJobNames)
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
