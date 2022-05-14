# Guess The Number Game 🎮

This simple game allows a player to guess a randomly generated number within a given number of rounds.

The steps below allow you to download and run the source code on your local machine.

In future, I would provide a simple executable that will allow you **run the game without having to install rust on your machine**.
You can follow these steps if you wish to try out the game anyways, or contribute to the game. Thanks🤩

### Setup
This game is written in Rust, and therefore would need rust to run.
Follow these [steps](https://doc.rust-lang.org/book/ch01-01-installation.html) to install and set up rust on your computer (Mac/Linux or Windows)

Run
`rustc --version` to check if you have succesfully installed rust.
Also run `cargo --version` to check if cargo was succesfully installed with Rust.<br/>
You will need cargo to run the game.

### Cloning the project
This repository contains mulitple projects but you only want to clone the 'Guess the number game'. (or you can create a fork to your own github and then clone the fork).

To clone this branch (ensure you have git installed on your computer):
* Navigate to your desired folder in terminal or git terminal.
* Copy and paste `git clone --single-branch --branch guess-game ,github-url>` in terminal.

This should clone this branch of the project to your computer.<br/>
Alternatively, learn how to clone this branch using Gitub Desktop [here](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/adding-and-cloning-repositories/cloning-and-forking-repositories-from-github-desktop).

### Contributing
If you intend to contribute to the repo.
After cloning the repo, follow these steps below to safely make a contribution
* Add the main repository as an upstream to your local repository using
```
git remote add upstream <remote-repo-url>
```
* Create a new branch based on the current branch. You can use
```
git checkout -b <new-branch-name>
```
* You will be switched to this new branch where you can make any changes you wish to make.
* After adding and commiting your changes. `git add` and `git commit`, checkout back to the original branch.<br/>
In this case that would be `git checkout guess-game`.
* Next, merge your edited branch to this branch.
`git merge <new-branch>`<br/>
This is only advised if the upstream is your own repository. If the upstream is the original repository, skip this step
* Next, you can push your changes to the remote repository using
```
git push upstream <branch-name>
```
* Finally, go to the upstream repository on your GitHub. You should be able to make a pull request which will allow your changes to be implemented in the main project.

### Playing the game
if you have the game installed and you have rust on your machine, you are ready to play!

Run `cargo build && cargo run` to build and run the game.