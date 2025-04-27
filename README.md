# Basic Git Branching Practice
This is a short practice to git branching. Refer to [Pro Git (2nd Edition)](https://git-scm.com/book/en/v2).
I appreciate Scott Chacon, Ben Straub, and numerous contributors for making this wonderful guide book.

## Refer to 'Pro Git (2nd Edition)' Git Branching (p. 63~69)
Make a new branch 'testing'
```
git branch testing
```

Switch 'HEAD' to 'testing' or 'master'
```
git checkout testing
git checkout master
```

## Refer to 'Pro Git (2nd Edition)' Basic Branching and Merging (p. 70~79)
### Overall process of merging
> Merge -> (Conflicts -> Edit -> Add -> Commit)

Conflicts may happen or not.

### Example
When you try to merge a new branch '*testing*' (added ```fn add```) **into** '*master*' branch (added ```fn substract```).
Conflicts occured because ```src/main.rs``` in 'testing' and 'master' have different functions with slightly different ```main``` functions.
You decided to manually edit the ```src/main.rs``` to contain both functions (```add``` and ```substract```) with modifications in ```main``` function.

1. Change **HEAD** to 'master' branch (default initial branch name).
```
git checkout master
```
2. Merge (name the branch to be merged).
```
git merge testing
```
3. If they conflict, you have the following two options:
- You may abort the merge.
```
git merge --abort
```
- Open the file with the conflicts and edit it manually.
```
<<<<<<<: this indicates the contents of 'master'
from ======= to >>>>>>>: indicates the contents of 'testing'
```
You should erase '<<<, ===, >>>' because they will be saved to a commit if you don't delete them.

4. After editing the file, git add to the edited file.
```
git add src/main.rs
```
5. If the conflicts were resolved, you can commit which finishes the merge process.
```
git commit (or use GUI such as Sourcetree)
```
6. Check whether there are no merged branches (p. 80).
```
git branch --no-merged
```
