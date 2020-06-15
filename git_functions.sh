#!/bin/bash

git_no_updates() { x=$(git status | grep 'nothing added to commit but untracked files present' | wc -l); [ $x -ne 0 ]; return $?; }
git_untracked() { x=$(git status | grep 'Untracked files' | wc -l); [ $x -gt 0 ]; return $?; }

git_check_untracked() {
    git_untracked || { echo "[info] No untracked files"; return 0; }
    [ $opt_force -eq 1 ] && echo "[warn] You have untracked files" && return 0
    git status
    echo "[error] You have untracked files, use --force" && exit 1 
}

ts() { date +"%Y-%m-%d %H:%M:%S"; }



