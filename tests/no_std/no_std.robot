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
    Wait For Line On Uart   [log][tg] waiting 3 milliseconds before enable task2
    Wait For Line On Uart   [log][tg] waiting the task1 to trigger
    FOR    ${i}    IN RANGE    1    4
        Wait For Line On Uart   [log][tg] millisecond ${i} ...
    END
    Wait For Line On Uart   [log][tg] Task2 is enabled

Test Mutex
    Compile No-Std Test     nrf52840dk_nrf52840
    Setup Renode    nrf52840
    Wait For Line On Uart   [log][mtx] t1 0 old val: 0 (tm 0)
    Wait For Line On Uart   [log][mtx] t1 0 new val: 1 (tm 0)
    Wait For Line On Uart   [log][mtx] t2 0 old val: 1 (tm 0)
    Wait For Line On Uart   [log][mtx] t2 0 new val: 2 (tm 0)
    Wait For Line On Uart   [log][mtx] t1 1 old val: 2 (tm 2)
    Wait For Line On Uart   [log][mtx] t1 1 new val: 3 (tm 2)
    Wait For Line On Uart   [log][mtx] t2 1 old val: 3 (tm 2)
    Wait For Line On Uart   [log][mtx] t2 1 new val: 4 (tm 2)

Test Channel
    Compile No-Std Test     nrf52840dk_nrf52840
    Setup Renode    nrf52840
    Wait For Line On Uart   [log][chan] receiver 0
    Wait For Line On Uart   [log][chan] producer 1
    Wait For Line On Uart   [log][chan] producer 2
    Wait For Line On Uart   [log][chan] receiver 2
    Wait For Line On Uart   [log][chan] pkt produce timestamp: 2

Test Semaphore
    Compile No-Std Test     nrf52840dk_nrf52840
    Setup Renode    nrf52840
    Wait For Line On Uart   [log][sem] task1 trying to take the resource at 0 ...
    Wait For Line On Uart   [log][sem] task1 took the resource and exit at 0
    Wait For Line On Uart   [log][sem] task2 trying to take the resource at 0 ...
    Wait For Line On Uart   [log][sem] task3 will wait 2 seconds to give the resource. Now is 1
    Wait For Line On Uart   [log][sem] task3 giving the resource at 3 ...
    Wait For Line On Uart   [log][sem] task2 took the resource at 3
    Wait For Line On Uart   [log][sem] task2 is giving the resource at 4 ...
    Wait For Line On Uart   [log][sem] task2 give the resource at 4

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
