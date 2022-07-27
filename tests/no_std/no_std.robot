*** Settings ***
Library         Process
Library         OperatingSystem
Suite Setup     Setup
Suite Teardown  Teardown
Test Setup      Reset Emulation
Test Teardown   Test Teardown
Resource        ${RENODEKEYWORDS}

*** Variables ***
${WITH_CLEAN}  0

*** Test Cases ***
Test Ping Pong ZBUS
    Compile No-Std Test     nrf52840dk_nrf52840
    Setup Renode    nrf52840
    FOR    ${i}    IN RANGE    0    3
        Wait For Line On Uart   [log][zbus] ${i}. publishing at pong channel...
        Wait For Line On Uart   [log][zbus] ${i}. published. Waiting data in ping channel...
        Wait For Line On Uart   [c]receive ball pos: <1, 2>
        Wait For Line On Uart   [log][zbus] ${i}. receive ball pos: <7, 12>
    END

Test Trigger
    Compile No-Std Test     nrf52840dk_nrf52840
    Setup Renode    nrf52840
    Wait For Line On Uart   [log][tg] waiting 3 seconds before enable task2
    Wait For Line On Uart   [log][tg] waiting the task1 to trigger
    FOR    ${i}    IN RANGE    1    4
        Wait For Line On Uart   [log][tg] second ${i}...
    END
    Wait For Line On Uart   [log][tg] Task2 is enabled

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
    IF    ${WITH_CLEAN} == 1
        Log To Console    "Cleaning..."
        ${result} =     Run Process    make clean     stdout=STDOUT   shell=yes
        Should Be Equal As Integers    ${result.rc}   0
    END
    Log To Console    "Compiling rs-lib..."
    ${result} =     Run Process    make rs-lib     stdout=STDOUT  shell=yes
    Should Be Equal As Integers    ${result.rc}   0
    Log To Console    "Compiling firmware..."
    ${result} =     Run Process     west build -b ${board}    stdout=STDOUT   shell=yes
    Should Be Equal As Integers    ${result.rc}   0
    Log To Console    "Compile end"
