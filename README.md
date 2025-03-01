## Rust `workflow` Action
This GitHub Action runs specified workflow on a pull request on a Rust language project, and calculates the  ```Fibonnacci number``` of numbers found in a pull request.

# Example workflow
```yml
name: Rust Workflow

on: 
  pull_request:
    branches:
      - main
permissions:
  pull-requests: write
  contents: read
          

jobs:
  build-and-run:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      repository-projects: write
    steps:
      - name: Checkout code
        uses: actions/checkout@v2
      - name: Build and run Rust code
        id: input
        uses: leghadjeu-christian/FibBot@v1
        with:
          enable_fib: true
          max_threshold: 50
          request_number: ${{ github.event.pull_request.number }}
          github_token: ${{ secrets.GITHUB_TOKEN }} 
          actor: ${{ github.actor }}
          repository:   ${{ github.repository }}
```



## Inputs
All inputs are necessary for this action else the default values will be used.
| Inuput Name | Description | Default |
| -- | -- | -- |
|  `enable_fib` | If set to "true" the action calculates the fibonacci number of numbers collected from a pull request and post's them in the comment section  of the pull request. | "true" |
| `max_threshold` | If set with an integer number,  then it calculates only the fibonacci numbers of the numbers collected from the pull request who's values are below the `max_threshold` | 100 |
| `github_token` | Contains the `GITHUB_TOKEN` with read and write permisions for all scopes | |
| `request_number` | Defines the pull request's number on which the workflow is to run | 1 |
| `actor` | Defines the repository owner on which the action is to run | |
| `repository` | Defines the repository name on which the action is trigerred | |



