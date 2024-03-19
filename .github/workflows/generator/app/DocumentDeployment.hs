module DocumentDeployment (step) where

import qualified Workflow.GitHub.Actions as GHA

step :: GHA.Step
step =
  GHA.namedAs "Deployment to Firebase Hosting (for Peridot branch)" $
    GHA.runStep "./.github/actions/deployment-doc-peridot"
