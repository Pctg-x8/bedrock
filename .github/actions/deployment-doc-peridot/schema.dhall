let GithubActions = https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let Params = { FirebaseToken : Text }
let step = \(params: Params) -> GithubActions.Step::{
    , name = "Deployment to Firebase Hosting (for Peridot branch)"
    , uses = Some "./.github/actions/deployment-doc-peridot"
    , env = Some (toMap { FIREBASE_TOKEN = params.FirebaseToken })
    }

in { Params, step }
