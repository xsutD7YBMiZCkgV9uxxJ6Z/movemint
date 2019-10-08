Libra Core is a fast evolving codebase. Furthermore crates like the Move language `vm` are not readily available as dependable libraries.

So for now we have a simple approach to make use of Libra Core: using a git submodule. The strategy is likely to change in the near future as we find better ways to deal with this challenge.

upstream (Libra Core)

local (your local machine)

remote (your GitHub-hosted repo)

downstream (the movemint repo)

-----

TODO:

git flow
ci / cd / devops
reproducible builds