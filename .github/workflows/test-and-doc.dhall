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

let Map = https://prelude.dhall-lang.org/Map/Type

let Map/keys = https://prelude.dhall-lang.org/Map/keys

let Map/mapValue = https://prelude.dhall-lang.org/Map/map

let Optional/default = https://prelude.dhall-lang.org/Optional/default

let Optional/fold = https://prelude.dhall-lang.org/Optional/fold

let List/map = https://prelude.dhall-lang.org/List/map

let List/concat = https://prelude.dhall-lang.org/List/concat

let JobMap = Map Text GithubActions.Job.Type

let JobModifier = GithubActions.Job.Type → GithubActions.Job.Type

let flattenSteps =
      λ(steps : List (List GithubActions.Step.Type)) →
        List/concat GithubActions.Step.Type steps

let prependStep =
      λ(step : GithubActions.Step.Type) →
      λ(steps : List GithubActions.Step.Type) →
        flattenSteps [ [ step ], steps ]

let Text/concatSep = https://prelude.dhall-lang.org/Text/concatSep

let DocumentDeployment = ../actions/deployment-doc-peridot/schema.dhall

let SlackNotifierAction =
      https://raw.githubusercontent.com/Pctg-x8/ci-notifications-post-invoker/master/schema.dhall

let Features = ./features.dhall

let serializeFeatures = Text/concatSep ","

let dependsAll =
      λ(deps : List Text) →
      λ(job : GithubActions.Job.Type) →
        job ⫽ { needs = Some deps }

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
                GithubActions.mkExpression
                  "needs.preconditions.outputs.begintime"
            , report_name = "Integrity Check"
            , mode = SlackNotifierAction.Mode.Branch
            }
        ⫽ { name = "Notify as Failure" }

let slackNotifySuccessStep =
      SlackNotifierAction.step
        { status = SlackNotifierAction.Status.Success
        , begintime =
            GithubActions.mkExpression "needs.preconditions.outputs.begintime"
        , report_name = "Integrity Check"
        , mode = SlackNotifierAction.Mode.Branch
        }

let faultableJob =
      λ(job : GithubActions.Job.Type) →
        let jobName = Optional/default Text "<unknown job>" job.name

        in    job
            ⫽ { steps =
                  flattenSteps
                    [ job.steps
                    , runStepsOnFailure
                        [ configureSlackNotification
                        , slackNotifyIfFailureStep jobName
                        ]
                    ]
              }

let jobName =
      λ(name : Text) →
      λ(job : GithubActions.Job.Type) →
        job ⫽ { name = Some name }

let jobRunner =
      λ(platform : GithubActions.RunnerPlatform) →
      λ(job : GithubActions.Job.Type) →
        job ⫽ { runs-on = platform }

let useRepositoryContent =
      λ(job : GithubActions.Job.Type) →
          job
        ⫽ { steps = prependStep (Checkout.stepv3 Checkout.Params::{=}) job.steps
          }

let useRust =
      λ(toolchain : Text) →
      λ(job : GithubActions.Job.Type) →
          job
        ⫽ { steps =
              prependStep
                ( InstallRust.step
                    InstallRust.Params::{ toolchain = Some toolchain }
                )
                job.steps
          }

let useNightlyRust = useRust "nightly"

let useStableRust = useRust "stable"

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

let singleStepJob =
      λ(step : GithubActions.Step.Type) →
        GithubActions.Job::{
        , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
        , steps = [ step ]
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

let checkFormat =
      applyJobModifiers
        [ faultableJob
        , useRepositoryContent
        , useStableRust
        , jobName "Check Format"
        ]
        ( singleStepJob
            GithubActions.Step::{
            , name = "check fmt"
            , run = Some "cargo fmt -- --check"
            }
        )

let simpleTestRustWithFeatures =
      λ(features : List Text) →
        RunCargo.step
          RunCargo.Params::{
          , command = "test"
          , args = Some "--features ${serializeFeatures features}"
          }

let simpleCheckRustWithFeatures =
      λ(features : List Text) →
        RunCargo.step
          RunCargo.Params::{
          , command = "check"
          , args = Some "--features ${serializeFeatures features}"
          }

let platformIndependentTest =
      applyJobModifiers
        [ faultableJob
        , useRepositoryContent
        , useStableRust
        , jobName "Run Tests (Platform Independent)"
        ]
        ( singleStepJob
            (simpleTestRustWithFeatures Features.PlatformIndependent)
        )

let platformDependentTests =
      { win32 =
          applyJobModifiers
            [ faultableJob
            , useRepositoryContent
            , useStableRust
            , jobRunner GithubActions.RunnerPlatform.windows-latest
            , jobName "Run Tests (Win32 Specific)"
            ]
            (singleStepJob (simpleCheckRustWithFeatures Features.Win32Specific))
      , unix =
          applyJobModifiers
            [ faultableJob
            , useRepositoryContent
            , useStableRust
            , jobName "Run Tests (Unix Specific)"
            ]
            (singleStepJob (simpleCheckRustWithFeatures Features.UnixSpecific))
      , mac =
          applyJobModifiers
            [ faultableJob
            , useRepositoryContent
            , useStableRust
            , jobRunner GithubActions.RunnerPlatform.macos-latest
            , jobName "Run Tests (Mac Specific)"
            ]
            (singleStepJob (simpleCheckRustWithFeatures Features.MacSpecific))
      }

let documentDeploymentStep =
      let docFeatures = serializeFeatures Features.ForDocumentation

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

      in  applyJobModifiers
            [ faultableJob
            , useRepositoryContent
            , useNightlyRust
            , requestIDTokenWritePermission
            , jobName "Deploy Latest Document"
            ]
            GithubActions.Job::{
            , runs-on = GithubActions.RunnerPlatform.ubuntu-latest
            , steps =
                flattenSteps
                  [ [ RunCargo.step
                        RunCargo.Params::{
                        , command = "rustdoc"
                        , args = Some
                            "--features ${docFeatures} -- --cfg docsrs"
                        , toolchain = Some "nightly"
                        }
                    ]
                  , deploymentSteps
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

let requireBefore =
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

let checkJobs =
        requireBefore
          (toMap { test = platformIndependentTest })
          ( toMap
              { test-win32 = platformDependentTests.win32
              , test-unix = platformDependentTests.unix
              , test-mac = platformDependentTests.mac
              }
          )
      # toMap { check-format = checkFormat }

let allJobs =
      requireBefore
        (toMap { preconditions })
        ( requireBefore
            checkJobs
            (toMap { document-deploy = documentDeploymentStep })
        )

in  GithubActions.Workflow::{
    , name = Some "Integrity Check"
    , on = GithubActions.On.Single GithubActions.UnparameterizedTrigger.push
    , permissions = Some (toMap { id-token = "write", contents = "read" })
    , jobs = requireBefore allJobs (toMap { report-success = reportSuccessJob })
    }
