## Rust `workflow` Action
This GitHub Action runs specified workflow on a pull request on a Rust language project, and calculates the  ```Fibonnacci number``` of numbers found in a pull request.

### Inputs
- enable_fib:
 If set to "true" the action calculates the fibonacci number of numbers collected from a pull request and post's them in the comment section  of the pull request.
- max_threshold: 
