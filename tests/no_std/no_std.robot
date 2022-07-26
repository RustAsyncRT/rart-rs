*** Settings ***
Library         Process
Library         OperatingSystem
Suite Setup     Setup
Suite Teardown  Teardown
Test Setup      Reset Emulation
Test Teardown   Teardown No-Std Test
Resource        ${RENODEKEYWORDS}

*** Test Cases ***
Test Ping Pong ZBUS
    Setup No-Std Test       zbus
    Compile No-Std Test     nrf52840dk_nrf52840
    Setup Renode    nrf52840

*** Keywords ***
Setup Renode
    [Arguments]     ${platform}
    Execute Command         mach create
    Execute Command         machine LoadPlatformDescription @platforms/cpus/${platform}.repl
    Execute Command         sysbus LoadELF @${CURDIR}/build/zephyr/zephyr.elf
    Create Terminal Tester  sysbus.uart0
    Start Emulation

Compile No-Std Test
    [Arguments]     ${board}
    Log To Console    "Cleaning..."
    ${result} =     Run Process    make clean     stdout=STDOUT   shell=yes
    Should Be Equal As Integers    ${result.rc}   0
    Log To Console    "Compiling rs-lib..."
    ${result} =     Run Process    make rs-lib     stdout=STDOUT  shell=yes
    Should Be Equal As Integers    ${result.rc}   0
    Log To Console    "Compiling firmware..."
    ${result} =     Run Process     west build -b ${board}    stdout=STDOUT   shell=yes
    Should Be Equal As Integers    ${result.rc}   0
    Log To Console    "Compile end"

Setup No-Std Test
    [Arguments]     ${filename}
    Copy File    rs_lib/src/${filename}.rs    rs_lib/src/lib.rs
    Copy File    src/${filename}.c    src/main.c

Teardown No-Std Test
    Test Teardown
    Remove File    rs_lib/src/lib.rs
    Remove File    src/main.c
