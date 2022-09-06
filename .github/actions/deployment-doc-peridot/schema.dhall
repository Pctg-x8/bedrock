let GithubActions =
      https://raw.githubusercontent.com/Pctg-x8/gha-schemas/master/schema.dhall

let step =
      GithubActions.Step::{
      , name = "Deployment to Firebase Hosting (for Peridot branch)"
      , uses = Some "./.github/actions/deployment-doc-peridot"
      }

in  { step }
