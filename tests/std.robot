*** Settings ***
Library     Process

*** Test Cases ***
Test basic.rs
    Run "basic" At "basic" Rust Test

*** Keywords ***
Run ${test} At ${file} Rust Test
    ${result} =     Run Process     cargo test --features "std" --package rart-rs --test ${file} ${test} -- --exact     shell=yes
