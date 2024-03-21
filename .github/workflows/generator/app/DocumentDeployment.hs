module DocumentDeployment (step) where

import qualified Data.Map as M
import qualified Workflow.GitHub.Actions as GHA

step :: GHA.Step
step =
  GHA.namedAs "Deployment to Firebase Hosting (for Peridot branch)" $
    GHA.actionStep "./.github/actions/deployment-doc-peridot" M.empty
